//! 通信模块的状态管理

use lazy_static::lazy_static;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

use super::types::{CommConfig, ConnectionState, PubState};

lazy_static! {
    /// 通信配置
    pub static ref COMM_CONFIG: Mutex<Option<CommConfig>> = Mutex::new(None);

    /// PULL 接收者运行标志
    pub static ref COMM_PULL_RUNNING: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

    /// PULL 状态（地址和运行标志）
    pub static ref PULL_STATE: Mutex<Option<PullState>> = Mutex::new(None);

    /// PUB 发布者运行标志
    pub static ref COMM_PUB_RUNNING: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

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
    pub address: String,
}

impl PullState {
    pub fn new(address: String) -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            address,
        }
    }
}

/// PUB 状态的只读视图
#[derive(Debug, Clone)]
pub struct PubStateView {
    pub connected: ConnectionState,
    pub messages_sent: u64,
    pub bytes_sent: u64,
    pub last_error: Option<String>,
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

/// 获取 PULL 运行状态
pub fn get_pull_running() -> bool {
    COMM_PULL_RUNNING
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()
                .map(|r| r.load(std::sync::atomic::Ordering::SeqCst))
        })
        .unwrap_or(false)
}

/// 设置 PULL 运行标志
pub fn set_pull_running(running: Option<Arc<AtomicBool>>) -> Result<(), String> {
    let mut guard = COMM_PULL_RUNNING.lock().map_err(|_| "Lock failed")?;
    *guard = running;
    Ok(())
}

/// 获取 PUB 运行状态
pub fn get_pub_running() -> bool {
    COMM_PUB_RUNNING
        .lock()
        .ok()
        .and_then(|g| {
            g.as_ref()
                .map(|r| r.load(std::sync::atomic::Ordering::SeqCst))
        })
        .unwrap_or(false)
}

/// 设置 PUB 运行标志
pub fn set_pub_running(running: Option<Arc<AtomicBool>>) -> Result<(), String> {
    let mut guard = COMM_PUB_RUNNING.lock().map_err(|_| "Lock failed")?;
    *guard = running;
    Ok(())
}

/// 获取 PUB 状态的只读视图
pub fn get_pub_state_view() -> Result<PubStateView, String> {
    let guard = COMM_PUB_STATE.lock().map_err(|_| "Lock failed")?;
    let state = guard
        .as_ref()
        .ok_or_else(|| "PUB state not initialized".to_string())?;
    Ok(PubStateView {
        connected: state.get_connection_state(),
        messages_sent: state.messages_sent(),
        bytes_sent: state.bytes_sent(),
        last_error: state.last_error(),
    })
}

/// 设置 PUB 状态
pub fn set_pub_state(state: PubState) -> Result<(), String> {
    let mut guard = COMM_PUB_STATE.lock().map_err(|_| "Lock failed")?;
    *guard = Some(state);
    Ok(())
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

/// 获取 PULL 地址
pub fn get_pull_address() -> Result<String, String> {
    let guard = PULL_STATE.lock().map_err(|_| "Lock failed")?;
    guard
        .as_ref()
        .map(|s| s.address.clone())
        .ok_or_else(|| "PULL state not initialized".to_string())
}

/// 设置 PULL 地址
pub fn set_pull_address(addr: String) -> Result<(), String> {
    let mut guard = PULL_STATE.lock().map_err(|_| "Lock failed")?;
    if let Some(ref mut state) = *guard {
        state.address = addr;
    } else {
        *guard = Some(PullState::new(addr));
    }
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
