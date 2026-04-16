//! ZMQ 发布者实现

use std::thread;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::observe::types::FrameMessage;

/// 创建 ZMQ PUB 发布线程
///
/// # Arguments
/// * `addr` - ZMQ 地址 (如 "tcp://127.0.0.1:5556")
/// * `running` - 运行标志，用于控制线程停止
/// * `receiver` - 帧消息接收器
///
/// # Returns
/// * `Sender<FrameMessage>` - 用于发送帧消息的发送端
pub fn start_publisher(
    addr: &str,
    running: Arc<AtomicBool>,
    receiver: Receiver<FrameMessage>,
) -> Result<(), String> {
    let addr = addr.to_string();

    thread::spawn(move || {
        let ctx = zmq::Context::new();
        let socket = match ctx.socket(zmq::PUB) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("ZMQ socket creation failed: {}", e);
                return;
            }
        };

        if let Err(e) = socket.bind(&addr) {
            eprintln!("ZMQ bind failed for {}: {}", addr, e);
            return;
        }

        let _ = socket.set_conflate(true);

        while running.load(Ordering::SeqCst) {
            match receiver.recv_timeout(std::time::Duration::from_millis(100)) {
                Ok(frame) => {
                    let json = match serde_json::to_string(&frame) {
                        Ok(j) => j,
                        Err(_) => continue,
                    };
                    if socket.send(json.as_bytes(), 0).is_err() {
                        break;
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // No message, continue checking running flag
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    });

    Ok(())
}