//! Think 模块 Tauri 命令

use std::sync::atomic::Ordering;

use crate::communication::config as comm_config;
use crate::communication::Puller;
use crate::think;
use crate::think::types::{DecisionLog, ThinkConfig};
use crate::think::PullState;
use tauri::Emitter;

#[tauri::command]
pub fn think_start(app: tauri::AppHandle) -> Result<(), String> {
    let mut pull_state = think::PULL_STATE
        .lock()
        .map_err(|e| e.to_string())?;

    if pull_state.is_some() {
        return Err("Think module is already running".to_string());
    }

    let addr = comm_config::get_think_pull_address()?;
    let puller = Puller::new(&addr)?;
    let running_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let app_for_callback = app.clone();

    let handle = puller.start(running_flag.clone(), move |data: String| {
        if let Ok(log) = serde_json::from_str::<DecisionLog>(&data) {
            if let Err(e) = think::config::add_log(log.clone()) {
                log::error!("Failed to add decision log: {}", e);
            }
            let _ = app_for_callback.emit("think:decision_log", &log);
        }
    })?;

    // 记录启动时间（用于 uptime 计算）
    think::config::set_uptime_start();

    // 关键修复：使用相同的 running 标志和 JoinHandle 创建 PullState
    *pull_state = Some(PullState::new(running_flag, handle));

    Ok(())
}

#[tauri::command]
pub fn think_stop() -> Result<(), String> {
    // 获取锁
    let mut pull_state = think::PULL_STATE
        .lock()
        .map_err(|e| e.to_string())?;

    // 1. 设置 running = false 通知线程停止
    if let Some(state) = pull_state.as_ref() {
        state.running.store(false, Ordering::SeqCst);
    }

    // 2. 使用 take() 取出 JoinHandle，以便在锁外 join
    let join_handle = pull_state
        .as_mut()
        .and_then(|state| state.join_handle.take());

    // 3. 清空 pull_state
    *pull_state = None;

    // 锁在这里自动释放

    // 4. 等待线程结束（不在持有锁时调用，避免死锁）
    if let Some(h) = join_handle {
        let _ = h.join();
    }

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