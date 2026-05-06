//! PUB 模块
//!
//! 向 Python 端发送图像帧数据

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

use crate::communication::types::{FrameMessage, FrameType, PubState};
use log::error;
use tauri::Emitter;

/// 创建 PUB 发布线程
///
/// # Arguments
/// * `addr` - 地址 (如 "tcp://127.0.0.1:5556")
/// * `running` - 运行标志，用于控制线程停止
/// * `stopped` - 停止标志，收到时立即停止发送
/// * `receiver` - 帧消息接收器
/// * `pub_state` - PUB 状态，用于更新连接状态和发送统计
/// * `app_handle` - Tauri AppHandle，用于向前端发送状态事件
///
/// # Returns
/// * `JoinHandle<()>` - 用于等待线程结束
pub fn start_publisher(
    addr: &str,
    running: Arc<AtomicBool>,
    stopped: Arc<AtomicBool>,
    receiver: Receiver<FrameMessage>,
    pub_state: Arc<Mutex<PubState>>,
    app_handle: tauri::AppHandle,
) -> Result<JoinHandle<()>, String> {
    let addr = addr.to_string();

    let handle = thread::spawn(move || {
        let ctx = zmq::Context::new();
        let socket = match ctx.socket(zmq::PUB) {
            Ok(s) => s,
            Err(e) => {
                error!("Socket creation failed: {}", e);
                if let Ok(state) = pub_state.lock() {
                    state.set_error(format!("Socket creation failed: {}", e));
                }
                return;
            }
        };

        if let Err(e) = socket.bind(&addr) {
            error!("Bind failed for {}: {}", addr, e);
            if let Ok(state) = pub_state.lock() {
                state.set_error(format!("Bind failed: {}", e));
            }
            return;
        }

        // 连接成功
        if let Ok(state) = pub_state.lock() {
            state.set_connected();
        }
        let _ = app_handle.emit("observe:pub_started", &addr);

        let _ = socket.set_conflate(true);

        while running.load(Ordering::SeqCst) {
            match receiver.recv_timeout(std::time::Duration::from_millis(100)) {
                Ok(frame) => {
                    // 如果已收到停止信号，不再发送任何数据
                    if stopped.load(Ordering::SeqCst) {
                        break;
                    }
                    // 如果收到停止帧类型，通知 Python 并退出
                    if frame.frame_type == FrameType::Stop {
                        break;
                    }
                    let json = match serde_json::to_string(&frame) {
                        Ok(j) => j,
                        Err(_) => continue,
                    };
                    let bytes = json.len() as u64;
                    match socket.send(json.as_bytes(), 0) {
                        Ok(_) => {
                            if let Ok(state) = pub_state.lock() {
                                state.add_messages_sent(1);
                                state.add_bytes_sent(bytes);
                            }
                        }
                        Err(e) => {
                            error!("Send failed: {}", e);
                            let _ = app_handle.emit("observe:pub_error", "帧传输发生错误");
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
        if let Ok(state) = pub_state.lock() {
            state.set_disconnected();
        }
    });

    Ok(handle)
}
