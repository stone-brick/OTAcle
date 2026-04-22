//! Think 模块 Tauri 命令

use std::sync::atomic::Ordering;

use crate::communication::config as comm_config;
use crate::think;
use crate::think::types::ThinkConfig;

#[tauri::command]
pub fn think_start(app: tauri::AppHandle) -> Result<(), String> {
    let mut running = think::THINK_PULL_RUNNING
        .lock()
        .map_err(|e| e.to_string())?;

    if running.is_some() {
        return Err("Think module is already running".to_string());
    }

    let addr = &comm_config::get_think_pull_address()?;
    let puller = think::DecisionPuller::new(addr)?;
    let running_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

    let handle = puller.start(running_flag.clone(), app)?;
    *running = Some(running_flag);

    // 注意：handle 被忽略，线程在后台运行
    let _ = handle;

    Ok(())
}

#[tauri::command]
pub fn think_stop() -> Result<(), String> {
    let mut running = think::THINK_PULL_RUNNING
        .lock()
        .map_err(|e| e.to_string())?;

    if let Some(flag) = running.as_ref() {
        flag.store(false, Ordering::SeqCst);
    }

    *running = None;

    think::config::reset_state();

    Ok(())
}

#[tauri::command]
pub fn think_get_status() -> Result<think::types::ThinkStatus, String> {
    Ok(think::config::get_status())
}

#[tauri::command]
pub fn think_load_config(path: String) -> Result<ThinkConfig, String> {
    think::config::load_config(&path)
}

#[tauri::command]
pub fn think_save_config(path: String, config: ThinkConfig) -> Result<(), String> {
    think::config::save_config(&path, &config)
}

#[tauri::command]
pub fn think_get_config() -> Result<ThinkConfig, String> {
    think::config::get_config()
}

#[tauri::command]
pub fn think_get_logs(limit: Option<usize>) -> Result<Vec<think::types::DecisionLog>, String> {
    think::config::get_logs(limit)
}