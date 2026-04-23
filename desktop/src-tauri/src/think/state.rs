//! Think 模块全局状态

use super::types::{DecisionLog, ThinkConfig};
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

lazy_static! {
    /// 决策日志历史（环形缓冲区）
    pub static ref DECISION_LOGS: Mutex<Vec<DecisionLog>> = Mutex::new(Vec::new());

    /// Think 模块配置
    pub static ref THINK_CONFIG: Mutex<Option<ThinkConfig>> = Mutex::new(None);

    /// 运行启动时间
    pub static ref UPTIME_START: Mutex<Option<Instant>> = Mutex::new(None);

    /// 收到的消息计数
    pub static ref RECEIVED_COUNT: AtomicU64 = AtomicU64::new(0);
}

/// PULL 状态（地址和运行标志）
pub struct PullState {
    pub running: Arc<AtomicBool>,
    pub join_handle: Option<std::thread::JoinHandle<()>>,
}

impl PullState {
    /// 使用外部提供的 running 和 join_handle 创建
    pub fn new(running: Arc<AtomicBool>, join_handle: std::thread::JoinHandle<()>) -> Self {
        Self {
            running,
            join_handle: Some(join_handle),
        }
    }
}

lazy_static! {
    /// Think PULL 运行时状态
    pub static ref PULL_STATE: Mutex<Option<PullState>> = Mutex::new(None);
}