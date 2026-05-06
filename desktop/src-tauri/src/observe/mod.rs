//! Observe 模块 - 窗口捕获和预处理，供 Python 端使用
//!
//! 本模块提供：
//! - 窗口截图捕获（通过 windows-capture 使用 Windows Graphics Capture API）
//! - 图像预处理（缩放、裁剪）
//! - ZMQ PUB 用于向 Python 发送帧
//! - Tauri 事件用于前端预览

pub mod capture;
pub mod config;
pub mod processor;
pub mod state;
pub mod status;
pub mod types;

pub use state::SESSIONS;

/// 停止所有会话
pub fn stop_all_sessions() -> Result<(), String> {
    let mut sessions = SESSIONS
        .lock()
        .map_err(|_| "Failed to lock sessions".to_string())?;

    // 设置 running = false 通知线程停止
    for (_hwnd, handle) in sessions.iter() {
        handle
            .running
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    // 使用 drain 取出所有 SessionHandle，以便在锁外 join
    let handles: Vec<(
        Option<std::thread::JoinHandle<()>>,
        Option<std::thread::JoinHandle<()>>,
    )> = sessions
        .drain()
        .map(|(_hwnd, mut handle)| {
            let capture = handle.capture_handle.take();
            let publisher = handle.publisher_handle.take();
            (capture, publisher)
        })
        .collect();

    // 更新全局统计
    if let Ok(mut global) = crate::observe::state::GLOBAL_STATS.lock() {
        global.active_sessions = 0;
    }

    // 清空全局 PUB 状态
    let _ = crate::communication::state::clear_pub_state_arc();

    // 等待所有线程结束
    for (capture_handle, publisher_handle) in handles {
        if let Some(h) = capture_handle {
            let _ = h.join();
        }
        if let Some(h) = publisher_handle {
            let _ = h.join();
        }
    }

    Ok(())
}
