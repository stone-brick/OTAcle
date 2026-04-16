//! Observe 模块 - 窗口捕获和预处理，供 Python 端使用
//!
//! 本模块提供：
//! - 窗口截图捕获（通过 windows-capture 使用 Windows Graphics Capture API）
//! - 图像预处理（缩放、裁剪）
//! - ZMQ PUB 用于向 Python 发送帧
//! - Tauri 事件用于前端预览

pub mod types;
pub mod config;
pub mod processor;
pub mod capture;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    /// Global observe session
    pub static ref OBSERVE_SESSION: Mutex<Option<crate::observe::capture::CaptureSession>> = Mutex::new(None);
}
