//! Observe 模块的状态查询 API
//!
//! 提供 observe_get_status 及相关查询函数

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::communication::types::ConnectionState;
use super::state::{GLOBAL_STATS, SESSIONS, ObserveGlobalStats, SessionHandle};
use super::types::ObserveConfig;

/// 会话状态详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatus {
    pub hwnd: isize,
    pub running: bool,
    pub uptime_seconds: u64,
    pub config: ObserveConfig,
    pub frames_captured: u64,
    pub bytes_sent: u64,
    pub errors_count: u64,
    pub sender_connected: bool,
    pub messages_sent: u64,
}

impl SessionStatus {
    pub fn from_handle(handle: &SessionHandle) -> Self {
        Self {
            hwnd: handle.hwnd,
            running: handle.is_running(),
            uptime_seconds: handle.uptime(),
            config: handle.config.clone(),
            frames_captured: handle.stats.frames_captured(),
            bytes_sent: handle.stats.bytes_sent(),
            errors_count: handle.stats.errors_count(),
            sender_connected: handle.pub_state.get_connection_state() == ConnectionState::Connected,
            messages_sent: handle.pub_state.messages_sent(),
        }
    }
}

/// Observe 完整状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserveStatus {
    pub sessions: HashMap<isize, SessionStatus>,
    pub global_stats: ObserveGlobalStats,
}

/// 获取完整的状态信息
pub fn get_full_status() -> Result<ObserveStatus, String> {
    let sessions = SESSIONS.lock().map_err(|_| "Failed to lock sessions".to_string())?;

    let mut session_statuses = HashMap::new();
    for (hwnd, handle) in sessions.iter() {
        session_statuses.insert(*hwnd, SessionStatus::from_handle(handle));
    }

    let global_stats = GLOBAL_STATS
        .lock()
        .map_err(|_| "Failed to lock global stats".to_string())?
        .clone();

    Ok(ObserveStatus {
        sessions: session_statuses,
        global_stats,
    })
}

/// 获取简化的状态映射（兼容旧 API）
pub fn get_status_map() -> Result<HashMap<isize, bool>, String> {
    let sessions = SESSIONS.lock().map_err(|_| "Failed to lock sessions".to_string())?;

    Ok(sessions
        .iter()
        .map(|(hwnd, handle)| (*hwnd, handle.is_running()))
        .collect())
}
