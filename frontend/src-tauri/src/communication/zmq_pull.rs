//! ZMQ PULL 模块
//!
//! 从 Python 端接收 ZeroMQ PUSH 发送的控制命令

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use zmq::{Context, Socket};

use crate::communication::types::ZmqCommand;

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
