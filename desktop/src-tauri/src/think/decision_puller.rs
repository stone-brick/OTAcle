//! ZMQ PULL 接收器 - 接收 Python 端发送的决策日志

use log::{debug, error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tauri::Emitter;
use zmq::{Context, Socket};

use crate::think::config::{add_log, set_uptime_start};
use crate::think::types::DecisionLog;

/// PULL 接收者
pub struct DecisionPuller {
    socket: Socket,
}

impl DecisionPuller {
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
    pub fn start(
        self,
        running: Arc<AtomicBool>,
        app: tauri::AppHandle,
    ) -> Result<thread::JoinHandle<()>, String> {
        running.store(true, Ordering::SeqCst);
        set_uptime_start();
        info!("Decision PULL listener started");

        let socket = self.socket;
        let running_clone = running.clone();

        Ok(thread::spawn(move || {
            while running_clone.load(Ordering::SeqCst) {
                match socket.recv_string(zmq::DONTWAIT) {
                    Ok(Ok(data)) => {
                        match serde_json::from_str::<DecisionLog>(&data) {
                            Ok(log) => {
                                debug!("PULL received decision log: step={}", log.step);
                                if let Err(e) = add_log(log.clone()) {
                                    error!("Failed to add decision log: {}", e);
                                }
                                // 发送事件到前端
                                let _ = app.emit("think:decision_log", &log);
                            }
                            Err(e) => {
                                error!("Failed to parse decision log: {}", e);
                                let _ = app.emit("think:error", serde_json::json!({"error": format!("Parse error: {}", e)}));
                            }
                        }
                    }
                    Ok(Err(_)) => {
                        debug!("Empty message received, ignored");
                    }
                    Err(e) => {
                        debug!("ZMQ receive would block: {}", e);
                    }
                }

                thread::sleep(std::time::Duration::from_millis(10));
            }
            info!("Decision PULL listener stopped");
        }))
    }
}