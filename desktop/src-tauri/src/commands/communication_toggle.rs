//! 一键启动/停止所有通信连接
//!
//! 同时控制 Act PULL 和 Observe PUB

use crate::act;
use crate::commands::observe::observe_start;
use crate::communication::types::Command;
use crate::communication::{state as comm_state, PullState, Puller};
use crate::think;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Emitter};

/// 保存的 Observe 启动信息（用于快捷键重启）
static SAVED_OBSERVE_INFO: Lazy<Mutex<Option<ObserveInfo>>> = Lazy::new(|| Mutex::new(None));

/// Observe 启动信息
struct ObserveInfo {
    window: String,
    config: crate::observe::types::ObserveConfig,
}

/// 保存 Observe 启动信息（供快捷键使用）
pub fn save_observe_info(window: String, config: crate::observe::types::ObserveConfig) {
    if let Ok(mut guard) = SAVED_OBSERVE_INFO.lock() {
        *guard = Some(ObserveInfo { window, config });
    }
}

/// 获取保存的 Observe 信息
pub fn get_saved_observe_info() -> Option<(String, crate::observe::types::ObserveConfig)> {
    let guard = SAVED_OBSERVE_INFO.lock().ok()?;
    guard.as_ref().map(|info| (info.window.clone(), info.config.clone()))
}

/// 清除保存的 Observe 信息
fn clear_observe_info() {
    if let Ok(mut guard) = SAVED_OBSERVE_INFO.lock() {
        *guard = None;
    }
}

/// 启动 Observe（使用保存的信息）
fn start_observe(app: AppHandle) -> Result<(), String> {
    let (window, config) = get_saved_observe_info().ok_or("No saved observe info")?;
    observe_start(window, config, app)
}

/// 切换 PULL 和 PUB 的运行状态
/// 如果都已停止则全部启动，如果有任何一项在运行则全部停止
#[tauri::command]
pub async fn comm_toggle_all(app: AppHandle) -> Result<(), String> {
    // 获取当前状态
    let act_running = comm_state::get_pull_state()
        .map(|s| s.running.load(Ordering::SeqCst))
        .unwrap_or(false);
    let think_running = think::PULL_STATE
        .lock()
        .map(|s| s.as_ref().map(|st| st.running.load(Ordering::SeqCst)).unwrap_or(false))
        .unwrap_or(false);
    let pub_running = comm_state::get_pub_running();

    if act_running || think_running || pub_running {
        // 有任何一项在运行，全部停止
        // 停止顺序: Act PULL → Think PULL → Observe PUB
        stop_pull().await?;
        stop_think()?;
        stop_observe().await?;
    } else {
        // 全部停止，启动全部
        // 启动顺序: Act PULL → Think PULL → Observe PUB
        start_pull(app.clone()).await?;
        start_think(app.clone())?;
        // 如果有保存的 Observe 信息，启动它
        if let Ok(()) = start_observe(app.clone()) {
            // Observe 启动成功
        }
    }

    Ok(())
}

/// 启动 PULL
async fn start_pull(app: AppHandle) -> Result<(), String> {
    // 获取 PULL 地址
    let addr = {
        let config = comm_state::get_config()
            .map_err(|_| "Communication config not loaded. Call comm_load_config first.")?;
        config.act_pull_address
    };

    // 创建新的 PULL 状态
    let pull_state = PullState::new();
    let running = pull_state.running.clone();
    let running_for_thread = running.clone();

    // 保存 PULL 状态
    comm_state::set_pull_state(pull_state)?;

    // 创建接收者
    let puller = Puller::new(&addr)?;
    let app_for_callback = app.clone();

    // 在独立线程中开始监听
    let _handle = puller.start(running_for_thread, move |data: String| {
        if let Ok(cmd) = serde_json::from_str::<Command>(&data) {
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
        }
    });

    Ok(())
}

/// 停止 PULL
async fn stop_pull() -> Result<(), String> {
    let mut state_guard = comm_state::PULL_STATE.lock().map_err(|_| "Lock failed")?;
    if let Some(ref mut pull_state) = *state_guard {
        pull_state.running.store(false, Ordering::SeqCst);
    }
    Ok(())
}

/// 停止 Observe（PUB）
async fn stop_observe() -> Result<(), String> {
    crate::observe::stop_all_sessions()
}

/// 启动 Think PULL
fn start_think(app: AppHandle) -> Result<(), String> {
    crate::commands::think::think_start(app)
}

/// 停止 Think PULL
fn stop_think() -> Result<(), String> {
    crate::commands::think::think_stop()
}

/// 获取切换状态（用于前端显示）
#[tauri::command]
pub fn comm_get_toggle_status() -> Result<(bool, bool, bool), String> {
    let act_pull_running = comm_state::get_pull_state()
        .map(|s| s.running.load(Ordering::SeqCst))
        .unwrap_or(false);
    let think_pull_running = think::PULL_STATE
        .lock()
        .map(|s| s.as_ref().map(|st| st.running.load(Ordering::SeqCst)).unwrap_or(false))
        .unwrap_or(false);
    let pub_running = comm_state::get_pub_running();
    Ok((act_pull_running, think_pull_running, pub_running))
}

/// 检查是否满足启动 Observe 的条件
/// 返回 (should_observe, window_spec, config_json)
/// - should_observe: true 表示有保存的配置或有目标窗口
/// - window_spec: 要使用的窗口规格字符串
/// - config_json: ObserveConfig 的 JSON 字符串
#[tauri::command]
pub fn comm_check_observe_start() -> Result<(bool, String, Option<String>), String> {
    // 1. 如果有保存的 Observe 信息，使用它
    if let Some((window, config)) = get_saved_observe_info() {
        let config_json = serde_json::to_string(&config).ok();
        return Ok((true, window, config_json));
    }

    // 2. 如果有目标窗口，使用它（前台窗口）
    if let Some(_hwnd) = act::config::get_target_window() {
        // 使用 "A" 表示前台窗口，config 使用默认配置
        let default_config = crate::observe::types::ObserveConfig::default();
        let config_json = serde_json::to_string(&default_config).ok();
        return Ok((true, "A".to_string(), config_json));
    }

    // 3. 没有可用的窗口信息，不能启动 Observe
    Ok((false, String::new(), None))
}