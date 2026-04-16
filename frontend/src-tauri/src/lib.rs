mod act;
mod input;
mod observe;
mod zmq_pub;
mod zmq_pull;
pub mod commands;
pub mod state;

use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

/// ZMQ 接收者全局状态
pub struct ZmqState {
    running: Mutex<Option<Arc<AtomicBool>>>,
    address: Mutex<String>,
}

impl Default for ZmqState {
    fn default() -> Self {
        Self {
            running: Mutex::new(None),
            address: Mutex::new("tcp://127.0.0.1:5555".to_string()),
        }
    }
}

/// 从窗口规格字符串获取 HWND
pub fn get_hwnd_from_spec(spec: &str) -> Result<isize, String> {
    let search = input::parse_window_spec(spec);

    // 空规格表示前台窗口
    if search.title.is_none()
        && search.class_name.is_none()
        && search.hwnd.is_none()
        && search.pid.is_none()
        && search.exe_name.is_none()
    {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.0.is_null() {
            return Err("No foreground window found".to_string());
        }
        return Ok(hwnd.0 as isize);
    }

    input::find_window(&search).ok_or_else(|| "Window not found".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ZmqState::default())
        .invoke_handler(tauri::generate_handler![
            // Action commands
            commands::action::load_action_config,
            commands::action::get_config,
            commands::action::get_action_list,
            commands::action::get_default_backend,
            commands::action::set_default_backend,
            commands::action::get_next_action_index,
            commands::action::create_action,
            commands::action::update_action,
            commands::action::delete_action,
            commands::action::save_action_config,
            commands::action::execute_action,
            commands::action::execute_action_with_params,
            commands::action::undo_action,
            commands::action::redo_action,
            commands::action::get_history_status,
            commands::action::clear_action_history,
            commands::action::discard_changes,
            commands::action::discard_action,
            // Target window commands
            commands::target_window::set_target_window,
            commands::target_window::get_target_window,
            // Execution backend commands
            commands::execution_backend::set_execution_backend,
            commands::execution_backend::get_execution_backend,
            // Window input commands
            commands::window_input::activate_window,
            commands::window_input::get_foreground_window,
            commands::window_input::get_window_info,
            commands::window_input::list_windows,
            commands::window_input::send_key,
            commands::window_input::send_text,
            commands::window_input::get_mouse_position,
            commands::window_input::send_mouse_click,
            commands::window_input::send_mouse_move,
            commands::window_input::send_combination_key,
            commands::window_input::activate_window_by_title,
            // Window search commands
            commands::window_search::find_window_by_title,
            commands::window_search::find_first_window_by_title,
            commands::window_search::find_windows_by_title,
            commands::window_search::find_first_window_by_title_contains,
            commands::window_search::find_windows_by_title_contains,
            commands::window_search::find_window_by_class,
            commands::window_search::find_windows_by_pid,
            commands::window_search::find_windows_by_exe,
            commands::window_search::find_window_by_hwnd,
            // ZMQ commands
            commands::zmq::act_zmq_start,
            commands::zmq::act_zmq_stop,
            commands::zmq::act_zmq_status,
            // Observe commands
            commands::observe::observe_start,
            commands::observe::observe_stop,
            commands::observe::observe_get_status,
            commands::observe::observe_save_config,
            commands::observe::observe_load_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
