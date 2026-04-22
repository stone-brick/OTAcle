//! 通信模块的状态管理

use lazy_static::lazy_static;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

use super::types::{CommConfig, ConnectionState, PubState};

lazy_static! {
    /// 通信配置
    pub static ref COMM_CONFIG: Mutex<Option<CommConfig>> = Mutex::new(None);

    /// PULL 状态（地址和运行标志）
    pub static ref PULL_STATE: Mutex<Option<PullState>> = Mutex::new(None);

    /// PUB 发布者状态
    pub static ref COMM_PUB_STATE: Mutex<Option<PubState>> = Mutex::new(None);

    /// PUB 状态的 Arc 引用（用于跨线程共享和查询）
    pub static ref COMM_PUB_STATE_ARC: Mutex<Option<std::sync::Arc<std::sync::Mutex<PubState>>>> =
        Mutex::new(None);
}

/// PULL 状态
#[derive(Clone)]
pub struct PullState {
    pub running: Arc<AtomicBool>,
}

impl PullState {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
        }
    }
}

/// 获取当前配置
pub fn get_config() -> Result<CommConfig, String> {
    let guard = COMM_CONFIG.lock().map_err(|_| "Lock failed")?;
    guard
        .clone()
        .ok_or_else(|| "Configuration not loaded".to_string())
}

/// 设置配置
pub fn set_config(config: CommConfig) -> Result<(), String> {
    let mut guard = COMM_CONFIG.lock().map_err(|_| "Lock failed")?;
    *guard = Some(config);
    Ok(())
}

/// 获取 PUB 运行状态
pub fn get_pub_running() -> bool {
    match COMM_PUB_STATE_ARC.lock() {
        Ok(guard) => guard
            .as_ref()
            .map(|s| {
                s.lock()
                    .map(|state| *state.connection_state.lock().unwrap() != ConnectionState::Disconnected)
                    .unwrap_or(false)
            })
            .unwrap_or(false),
        Err(_) => false,
    }
}

/// 获取 PULL 状态
pub fn get_pull_state() -> Result<PullState, String> {
    let guard = PULL_STATE.lock().map_err(|_| "Lock failed")?;
    guard
        .clone()
        .ok_or_else(|| "PULL state not initialized".to_string())
}

/// 设置 PULL 状态
pub fn set_pull_state(state: PullState) -> Result<(), String> {
    let mut guard = PULL_STATE.lock().map_err(|_| "Lock failed")?;
    *guard = Some(state);
    Ok(())
}

/// 设置 PUB 状态的 Arc 引用
pub fn set_pub_state_arc(state: std::sync::Arc<std::sync::Mutex<PubState>>) -> Result<(), String> {
    let mut guard = COMM_PUB_STATE_ARC
        .lock()
        .map_err(|_| "Lock failed")?;
    *guard = Some(state);
    Ok(())
}

/// 获取 PUB 状态的 Arc 引用
pub fn get_pub_state_arc() -> Result<std::sync::Arc<std::sync::Mutex<PubState>>, String> {
    let guard = COMM_PUB_STATE_ARC
        .lock()
        .map_err(|_| "Lock failed")?;
    guard
        .clone()
        .ok_or_else(|| "PUB state not initialized".to_string())
}

/// 清空 PUB 状态的 Arc 引用
pub fn clear_pub_state_arc() -> Result<(), String> {
    let mut guard = COMM_PUB_STATE_ARC
        .lock()
        .map_err(|_| "Lock failed")?;
    *guard = None;
    Ok(())
}
