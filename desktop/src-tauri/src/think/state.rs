//! Think 模块全局状态

use super::types::{DecisionLog, ThinkConfig};
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::{Arc, Mutex};
use std::time::Instant;

lazy_static! {
    /// 决策日志历史（环形缓冲区）
    pub static ref DECISION_LOGS: Mutex<Vec<DecisionLog>> = Mutex::new(Vec::new());

    /// Think 模块配置
    pub static ref THINK_CONFIG: Mutex<Option<ThinkConfig>> = Mutex::new(None);

    /// Think PULL 运行标志
    pub static ref THINK_PULL_RUNNING: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);
}

/// 收到的消息计数
pub static RECEIVED_COUNT: AtomicU64 = AtomicU64::new(0);

/// 运行启动时间
pub static UPTIME_START: Mutex<Option<Instant>> = Mutex::new(None);