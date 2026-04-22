mod act;
pub mod commands;
mod communication;
mod input;
mod observe;
mod project;
mod think;

use tauri_plugin_log::{Builder as LogBuilder, Target, TargetKind};

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
    // 初始化加载最近项目
    let _ = project::load_recent_projects();

    // 初始化通信模块的默认配置
    let _ = communication::state::set_config(communication::types::CommConfig::default());

    // 初始化 Think 模块的默认配置
    let _ = think::config::set_config(think::types::ThinkConfig::default());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(
            LogBuilder::default()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
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
            // Communication commands
            commands::communication::comm_load_config,
            commands::communication::comm_save_config,
            commands::communication::comm_get_config,
            commands::communication::comm_set_pull_address,
            commands::communication::comm_get_pull_address,
            commands::communication::comm_get_pub_address,
            commands::communication::comm_set_pub_address,
            commands::communication::comm_get_status,
            commands::communication_pull::comm_start_pull,
            commands::communication_pull::comm_stop_pull,
            // Observe commands
            commands::observe::observe_start,
            commands::observe::observe_stop,
            commands::observe::observe_get_status,
            commands::observe::observe_get_config,
            commands::observe::observe_add_crop_region,
            commands::observe::observe_remove_crop_region,
            commands::observe::observe_save_config,
            commands::observe::observe_load_config,
            commands::observe::observe_capture_preview,
            commands::observe::observe_capture_full_frame,
            // Project commands
            commands::project::project_open,
            commands::project::project_create,
            commands::project::project_close,
            commands::project::project_get_current,
            commands::project::project_get_recent,
            commands::project::project_remove_recent,
            commands::project::project_toggle_pin,
            commands::project::project_list_templates,
            commands::project::project_get_config_dir,
            commands::project::project_get_actions_config_path,
            commands::project::project_get_observe_config_path,
            commands::project::project_get_comm_config_path,
            commands::project::project_get_think_config_path,
            // Think commands
            commands::think::think_start,
            commands::think::think_stop,
            commands::think::think_get_status,
            commands::think::think_load_config,
            commands::think::think_save_config,
            commands::think::think_get_config,
            commands::think::think_get_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
