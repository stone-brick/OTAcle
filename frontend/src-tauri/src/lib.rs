mod act;
mod input;
mod observe;
pub mod commands;

use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

/// 从窗口规格字符串获取 HWND
pub fn get_hwnd_from_spec(spec: &str) -> Result<isize, String> {
    let search = input::parse_window_spec(spec);

    // 空规格表示前台窗口
    if search.title.is_none()
        && search.class_name.is_none()
        && search.hwnd.is_none()
        && search.pid.is_none()
        && search.process_name.is_none()
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
        .invoke_handler(tauri::generate_handler![
            // Action commands
            commands::action::act_load_config,
            commands::action::act_get_list,
            commands::action::act_get_default_backend,
            commands::action::act_set_default_backend,
            commands::action::act_get_next_index,
            commands::action::act_create,
            commands::action::act_update,
            commands::action::act_delete,
            commands::action::act_save_config,
            commands::action::act_execute_action,
            commands::action::act_execute_action_with_params,
            commands::action::act_undo,
            commands::action::act_redo,
            commands::action::act_get_history_status,
            commands::action::act_clear_history,
            commands::action::act_discard_all,
            // Target window commands
            commands::target_window::window_set_target,
            commands::target_window::window_get_target,
            // Execution backend commands
            commands::execution_backend::act_set_execution_backend,
            commands::execution_backend::act_get_execution_backend,
            // Window input commands
            commands::window_input::window_activate,
            commands::window_input::window_get_foreground,
            commands::window_input::window_get_info,
            commands::window_input::window_list,
            commands::window_input::window_send_key,
            commands::window_input::window_send_text,
            commands::window_input::window_get_mouse,
            commands::window_input::window_send_click,
            commands::window_input::window_send_move,
            commands::window_input::window_send_combo,
            commands::window_input::window_activate_by_title,
            // Window search commands
            commands::window_search::window_find_by_title,
            commands::window_search::window_find_one,
            commands::window_search::window_find_many,
            commands::window_search::window_find_one_contains,
            commands::window_search::window_find_many_contains,
            commands::window_search::window_find_by_class,
            commands::window_search::window_find_by_pid,
            commands::window_search::window_find_by_exe,
            commands::window_search::window_find_by_hwnd,
            // ZMQ commands
            commands::zmq::zmq_start,
            commands::zmq::zmq_stop,
            commands::zmq::zmq_get_status,
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
