use crate::act;

#[tauri::command]
pub fn window_set_target(window: Option<String>) -> Result<(), String> {
    act::config::set_target_window(window)
}

#[tauri::command]
pub fn window_get_target() -> Result<Option<i64>, String> {
    Ok(act::config::get_target_window().map(|hwnd| hwnd as i64))
}
