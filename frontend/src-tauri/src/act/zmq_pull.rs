//! ZMQ PULL 模块
//!
//! 接收 Python 通过 ZeroMQ PUSH 发送的控制命令

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use zmq::{Context, Socket};

/// ZMQ 命令消息格式
#[derive(Debug, Clone, Deserialize)]
pub struct ZmqCommand {
    /// 执行列表，按顺序对应配置文件中 action 的 index
    /// 例如 [true, false, true] 表示执行 index=0 和 index=2 的动作
    pub execute: Vec<bool>,
    /// 动态参数映射 (param_name -> value)
    /// 如果省略或为空，使用配置文件中的默认值
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

/// ZMQ 接收者
pub struct ZmqPuller {
    socket: Socket,
}

impl ZmqPuller {
    /// 创建新的 ZMQ 接收者
    pub fn new(addr: &str) -> Result<Self, String> {
        let ctx = Context::new();
        let socket = ctx
            .socket(zmq::PULL)
            .map_err(|e| format!("Failed to create socket: {}", e))?;

        socket
            .bind(addr)
            .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

        Ok(Self { socket })
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

        // 先发送一个初始化完成信号

        thread::spawn(move || {
            while running_clone.load(Ordering::SeqCst) {
                // 尝试接收消息
                match socket.recv_string(zmq::DONTWAIT) {
                    Ok(Ok(data)) => {
                        if let Ok(cmd) = serde_json::from_str::<ZmqCommand>(&data) {
                            on_message(cmd);
                        }
                    }
                    Ok(Err(_)) => {
                        // 空消息，忽略
                    }
                    Err(_e) => {
                        // 没有消息，忽略
                    }
                }

                thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        Ok(())
    }
}
