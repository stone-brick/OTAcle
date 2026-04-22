//! 通信模块的 Tauri 命令层
//!
//! 提供给前端的通信配置和状态管理接口

use crate::communication as comm;
use crate::communication::types::CommConfig;

#[tauri::command]
pub fn comm_load_config(path: String) -> Result<CommConfig, String> {
    comm::config::load_config(&path)
}

#[tauri::command]
pub fn comm_save_config(path: String, config: CommConfig) -> Result<(), String> {
    comm::config::save_config(&path, &config)
}

#[tauri::command]
pub fn comm_get_config() -> Result<CommConfig, String> {
    comm::config::get_config()
}

#[tauri::command]
pub fn comm_set_pull_address(addr: String) -> Result<(), String> {
    comm::config::set_act_pull_address(addr)
}

#[tauri::command]
pub fn comm_get_pub_address() -> Result<String, String> {
    comm::config::get_observe_pub_address()
}

#[tauri::command]
pub fn comm_get_pull_address() -> Result<String, String> {
    comm::config::get_act_pull_address()
}

#[tauri::command]
pub fn comm_set_pub_address(addr: String) -> Result<(), String> {
    comm::config::set_observe_pub_address(addr)
}

#[derive(serde::Serialize)]
pub struct CommStatus {
    pub pull_running: bool,
    pub pub_running: bool,
    pub pull_address: String,
    pub pub_address: String,
    pub pub_connected: String,
    pub pub_messages_sent: u64,
    pub pub_bytes_sent: u64,
    pub pub_last_error: Option<String>,
}

#[tauri::command]
pub fn comm_get_status() -> Result<CommStatus, String> {
    let config = comm::config::get_config().unwrap_or_default();

    let pull_running = comm::state::get_pull_state()
        .map(|s| s.running.load(std::sync::atomic::Ordering::SeqCst))
        .unwrap_or(false);
    let pub_running = comm::state::get_pub_running();

    let (pub_connected, pub_messages_sent, pub_bytes_sent, pub_last_error) =
        match comm::state::get_pub_state_arc() {
            Ok(state) => {
                use crate::communication::types::ConnectionState;
                let s = state.lock().map_err(|_| "Lock failed")?;
                let connected_str = match s.get_connection_state() {
                    ConnectionState::Connected => "connected",
                    ConnectionState::Disconnected => "disconnected",
                    ConnectionState::Error => "error",
                };
                (
                    connected_str.to_string(),
                    s.messages_sent(),
                    s.bytes_sent(),
                    s.last_error(),
                )
            }
            Err(_) => ("disconnected".to_string(), 0, 0, None),
        };

    Ok(CommStatus {
        pull_running,
        pub_running,
        pull_address: config.act_pull_address,
        pub_address: config.observe_pub_address,
        pub_connected,
        pub_messages_sent,
        pub_bytes_sent,
        pub_last_error,
    })
}
