//! ZMQ PUB 模块
//!
//! 向 Python 端通过 ZeroMQ PUB 发送图像帧数据

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

use crate::communication::types::{FrameMessage, ZmqPubState};

/// 创建 ZMQ PUB 发布线程
///
/// # Arguments
/// * `addr` - ZMQ 地址 (如 "tcp://127.0.0.1:5556")
/// * `running` - 运行标志，用于控制线程停止
/// * `receiver` - 帧消息接收器
/// * `zmq_state` - ZMQ 状态，用于更新连接状态和发送统计
///
/// # Returns
/// * `JoinHandle<()>` - 用于等待线程结束
pub fn start_publisher(
    addr: &str,
    running: Arc<AtomicBool>,
    receiver: Receiver<FrameMessage>,
    zmq_state: Arc<Mutex<ZmqPubState>>,
) -> Result<JoinHandle<()>, String> {
    let addr = addr.to_string();

    let handle = thread::spawn(move || {
        let ctx = zmq::Context::new();
        let socket = match ctx.socket(zmq::PUB) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("ZMQ socket creation failed: {}", e);
                if let Ok(mut state) = zmq_state.lock() {
                    state.set_error(format!("Socket creation failed: {}", e));
                }
                return;
            }
        };

        if let Err(e) = socket.bind(&addr) {
            eprintln!("ZMQ bind failed for {}: {}", addr, e);
            if let Ok(mut state) = zmq_state.lock() {
                state.set_error(format!("Bind failed: {}", e));
            }
            return;
        }

        // 连接成功
        if let Ok(mut state) = zmq_state.lock() {
            state.set_connected();
        }

        let _ = socket.set_conflate(true);

        while running.load(Ordering::SeqCst) {
            match receiver.recv_timeout(std::time::Duration::from_millis(100)) {
                Ok(frame) => {
                    let json = match serde_json::to_string(&frame) {
                        Ok(j) => j,
                        Err(_) => continue,
                    };
                    let bytes = json.len() as u64;
                    if socket.send(json.as_bytes(), 0).is_ok() {
                        if let Ok(mut state) = zmq_state.lock() {
                            state.add_messages_sent(1);
                            state.add_bytes_sent(bytes);
                        }
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

        // 线程结束，设置断开状态
        if let Ok(mut state) = zmq_state.lock() {
            state.set_disconnected();
        }
    });

    Ok(handle)
}
