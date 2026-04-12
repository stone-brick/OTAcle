//! ZMQ SUB 模块 - 接收 Python 发送的控制命令

use crate::action::types::ParamValue;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use zmq::{Context, Socket};

/// 自定义反序列化：省略时为 None，空对象 {} 报错，不允许 null
fn deserialize_optional_params<'de, D>(deserializer: D) -> Result<Option<HashMap<String, ParamValue>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MaybeMap {
        Map(HashMap<String, ParamValue>),
        Null,
    }

    let opt = Option::<MaybeMap>::deserialize(deserializer)?;
    match opt {
        None => Ok(None), // 省略字段
        Some(MaybeMap::Null) => Err(serde::de::Error::custom("params cannot be null, omit the field instead")),
        Some(MaybeMap::Map(map)) if map.is_empty() => {
            Err(serde::de::Error::custom("params cannot be empty object, omit the field or provide non-empty object"))
        }
        Some(MaybeMap::Map(map)) => Ok(Some(map)),
    }
}

/// ZMQ 命令消息格式
#[derive(Debug, Clone, Deserialize)]
pub struct ZmqCommand {
    /// 执行列表，按顺序对应配置文件中 action 的 index
    /// 例如 [true, false, true] 表示执行 index=0 和 index=2 的动作
    pub execute: Vec<bool>,
    /// 占位符参数映射（可选，省略为 None，不允许 null 或空对象）
    #[serde(default, deserialize_with = "deserialize_optional_params")]
    pub params: Option<HashMap<String, ParamValue>>,
}

/// ZMQ 订阅者
pub struct ZmqSubscriber {
    context: Context,
    socket: Socket,
}

impl ZmqSubscriber {
    /// 创建新的 ZMQ 订阅者
    pub fn new(addr: &str) -> Result<Self, String> {
        let ctx = Context::new();
        let socket = ctx
            .socket(zmq::SUB)
            .map_err(|e| format!("Failed to create socket: {}", e))?;

        socket
            .connect(addr)
            .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

        // 订阅所有消息（空字符串表示全部）
        socket
            .set_subscribe(b"")
            .map_err(|e| format!("Failed to subscribe: {}", e))?;

        Ok(Self {
            context: ctx,
            socket,
        })
    }

    /// 启动监听循环
    /// running: 原子布尔标志，用于控制线程停止
    /// on_message: 收到消息时的回调
    pub fn start<F>(self, running: Arc<AtomicBool>, on_message: F) -> Result<(), String>
    where
        F: Fn(ZmqCommand) + Send + 'static,
    {
        running.store(true, Ordering::SeqCst);

        let socket = self.socket;
        let running_clone = running.clone();

        thread::spawn(move || {
            while running_clone.load(Ordering::SeqCst) {
                // 使用 zmq::DONTWAIT 非阻塞接收
                match socket.recv_string(zmq::DONTWAIT) {
                    Ok(Ok(data)) => {
                        if let Ok(cmd) = serde_json::from_str::<ZmqCommand>(&data) {
                            on_message(cmd);
                        } else {
                            eprintln!("[ZMQ SUB] Failed to parse message: {}", data);
                        }
                    }
                    Ok(Err(_)) => {
                        // 非阻塞模式下没有消息是正常的
                        thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Err(_) => {
                        // 没有消息时短暂休眠
                        thread::sleep(std::time::Duration::from_millis(10));
                    }
                }
            }
        });

        Ok(())
    }
}
