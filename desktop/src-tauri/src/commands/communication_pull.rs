//! Communication 模块的 PULL 控制命令
//!
//! 提供给前端控制 PULL 连接

use crate::act;
use crate::communication::types::Command;
use crate::communication::{state as comm_state, PullState, Puller};
use std::sync::atomic::Ordering;
use tauri::Emitter;

#[tauri::command]
pub fn comm_start_pull(app: tauri::AppHandle) -> Result<(), String> {
    // 从 communication 状态获取 PULL 地址
    let addr = {
        let config = comm_state::get_config()
            .map_err(|_| "Communication config not loaded. Call comm_load_config first.")?;
        config.act_pull_address
    };

    // 停止现有的接收者（如果有）
    {
        let mut state_guard = comm_state::PULL_STATE.lock().map_err(|_| "Lock failed")?;
        if let Some(ref mut pull_state) = *state_guard {
            pull_state.running.store(false, Ordering::SeqCst);
        }
    }

    // 创建新的 PULL 状态
    let pull_state = PullState::new(addr.clone());
    let running = pull_state.running.clone();
    let running_for_thread = running.clone();

    // 保存 PULL 状态
    comm_state::set_pull_state(pull_state)?;

    // 克隆 app handle 用于回调
    let app_for_callback = app.clone();

    // 创建接收者
    let puller = Puller::new(&addr)?;

    // 在独立线程中开始监听
    puller.start(running_for_thread, move |cmd: Command| {
        let default_backend =
            act::config::get_default_backend().unwrap_or(act::types::InputBackend::Win32);

        let result = act::executor::execute_actions(cmd.execute, cmd.params, default_backend);

        match result {
            Ok(()) => {
                let msg = "[COMM] Actions executed successfully".to_string();
                let _ = app_for_callback.emit("comm:log", msg);
            }
            Err(e) => {
                let msg = format!("[COMM] Error: {}", e);
                let _ = app_for_callback.emit("comm:error", msg);
            }
        }
    })?;

    Ok(())
}

#[tauri::command]
pub fn comm_stop_pull() -> Result<(), String> {
    // 停止现有的接收者
    {
        let mut state_guard = comm_state::PULL_STATE.lock().map_err(|_| "Lock failed")?;
        if let Some(ref mut pull_state) = *state_guard {
            pull_state.running.store(false, Ordering::SeqCst);
        }
    }

    Ok(())
}
