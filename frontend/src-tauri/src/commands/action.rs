use crate::act;
use act::types::{Action, InputBackend};
use std::collections::HashMap;
use serde_json::Value;

#[tauri::command]
pub fn load_action_config(path: String, backend: Option<String>) -> Result<(), String> {
    let backend = match backend.as_deref() {
        Some("win32") => InputBackend::Win32,
        _ => InputBackend::Enigo,
    };
    act::config::load_config_with_backend(&path, backend)
}

#[tauri::command]
pub fn get_config() -> Result<act::types::ActionConfig, String> {
    act::config::get_config()
}

#[tauri::command]
pub fn get_action_list() -> Result<act::types::ActionConfigList, String> {
    act::config::get_action_list()
}

#[tauri::command]
pub fn get_default_backend() -> Result<InputBackend, String> {
    act::config::get_default_backend()
}

#[tauri::command]
pub fn set_default_backend(backend: InputBackend) -> Result<(), String> {
    act::config::set_default_backend(backend)
}

#[tauri::command]
pub fn get_next_action_index() -> Result<u32, String> {
    act::config::get_next_available_index()
}

#[tauri::command]
pub fn create_action(action: Action, name: Option<String>) -> Result<u32, String> {
    act::config::create_action(action, name)
}

#[tauri::command]
pub fn update_action(index: u32, action: Action, name: Option<String>) -> Result<(), String> {
    act::config::update_action(index, action, name)
}

#[tauri::command]
pub fn delete_action(index: u32) -> Result<(), String> {
    act::config::delete_action(index)
}

#[tauri::command]
pub fn save_action_config(
    path: String,
    default_backend: InputBackend,
    actions: act::types::ActionConfigList,
) -> Result<(), String> {
    act::config::save_config(&path, default_backend, &actions)
}

#[tauri::command]
pub fn execute_action(action_id: u32) -> Result<(), String> {
    let default_backend = act::config::get_default_backend()
        .unwrap_or(InputBackend::Win32);
    act::executor::execute_action(action_id, default_backend)
}

#[tauri::command]
pub fn execute_action_with_params(
    action_id: u32,
    params: HashMap<String, Value>,
) -> Result<(), String> {
    let default_backend = act::config::get_default_backend()
        .unwrap_or(InputBackend::Win32);
    act::executor::execute_action_with_params(action_id, params, default_backend)
}

#[tauri::command]
pub fn undo_action() -> Result<(), String> {
    act::config::undo()
}

#[tauri::command]
pub fn redo_action() -> Result<(), String> {
    act::config::redo()
}

#[tauri::command]
pub fn get_history_status() -> Result<(usize, usize), String> {
    Ok(act::config::get_history_status())
}

#[tauri::command]
pub fn clear_action_history() -> Result<(), String> {
    act::config::clear_history()
}

#[tauri::command]
pub fn discard_changes() -> Result<(), String> {
    act::config::discard_changes()
}

#[tauri::command]
pub fn discard_action(index: u32) -> Result<(), String> {
    act::config::discard_action(index)
}
