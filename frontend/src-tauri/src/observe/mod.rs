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
pub mod state;
pub mod status;

pub use capture::capture_screenshot;
pub use state::{GLOBAL_STATS, SESSIONS, SessionHandle};
pub use status::{get_full_status, get_status_map, ObserveStatus, SessionStatus};
pub use config::{load_config, save_config, get_config, validate_config,
                 update_config, add_crop_region, remove_crop_region};
