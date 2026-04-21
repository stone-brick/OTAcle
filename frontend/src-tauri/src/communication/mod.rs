//! 通信模块
//!
//! 统一管理项目中的数据通信
//! - PULL: 从 Python 接收动作执行命令（Act 模块使用）
//! - PUB: 向 Python 发送图像帧数据（Observe 模块使用）

pub mod config;
pub mod publisher;
pub mod puller;
pub mod state;
pub mod types;

pub use config::get_pub_address;
pub use publisher::start_publisher;
pub use puller::Puller;
pub use state::PullState;
