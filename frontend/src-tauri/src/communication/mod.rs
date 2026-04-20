//! 通信模块
//!
//! 统一管理项目中的数据通信
//! - PULL: 从 Python 接收动作执行命令（Act 模块使用）
//! - PUB: 向 Python 发送图像帧数据（Observe 模块使用）

pub mod config;
pub mod state;
pub mod types;
pub mod puller;
pub mod publisher;

pub use config::{get_config, get_pull_address, get_pub_address, load_config, save_config, set_pull_address, set_pub_address};
pub use state::{get_config as state_get_config, get_pub_running, get_pub_state_view, get_pull_running, get_pull_state, set_config as state_set_config, set_pub_running, set_pub_state, set_pull_running, set_pull_state as state_set_pull_state, PullState};
pub use types::{CommConfig, CropBlock, FrameMessage, Command, ConnectionState, PubState};
pub use puller::Puller;
pub use publisher::start_publisher;