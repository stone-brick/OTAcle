use crate::act;

#[tauri::command]
pub fn set_target_window(window: Option<String>) -> Result<(), String> {
    act::config::set_target_window(window)
}

#[tauri::command]
pub fn get_target_window() -> Result<Option<i64>, String> {
    Ok(act::config::get_target_window().map(|hwnd| hwnd as i64))
}
