//! PULL 模块
//!
//! 从 Python 端接收控制命令

use log::{debug, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use zmq::{Context, Socket};

use crate::communication::types::Command;

/// PULL 接收者
pub struct Puller {
    socket: Socket,
}

impl Puller {
    /// 创建新的 PULL 接收者
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
        F: Fn(Command) + Send + 'static,
    {
        running.store(true, Ordering::SeqCst);
        info!("PULL listener started");

        let socket = self.socket;
        let running_clone = running.clone();

        thread::spawn(move || {
            while running_clone.load(Ordering::SeqCst) {
                // 尝试接收消息
                match socket.recv_string(zmq::DONTWAIT) {
                    Ok(Ok(data)) => {
                        if let Ok(cmd) = serde_json::from_str::<Command>(&data) {
                            debug!("PULL received command: {} actions", cmd.execute.len());
                            on_message(cmd);
                        }
                    }
                    Ok(Err(_)) => {
                        // 空消息，忽略
                        debug!("Empty message received, ignored");
                    }
                    Err(e) => {
                        // 没有消息，忽略（EAGAIN 是正常情况）
                        debug!("ZMQ receive would block: {}", e);
                    }
                }

                thread::sleep(std::time::Duration::from_millis(10));
            }
            info!("PULL listener stopped");
        });

        Ok(())
    }
}
