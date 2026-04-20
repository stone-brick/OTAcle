use crate::act;
use act::types::{ActionData, ActionList, InputBackend};
use std::collections::HashMap;
use serde_json::Value;

#[tauri::command]
pub fn act_load_config(path: String, backend: Option<String>) -> Result<(), String> {
    let backend = match backend.as_deref() {
        Some("win32") => InputBackend::Win32,
        _ => InputBackend::Enigo,
    };
    act::config::load_config_with_backend(&path, backend)
}

#[tauri::command]
pub fn act_get_list() -> Result<ActionList, String> {
    act::config::get_action_list()
}

#[tauri::command]
pub fn act_get_default_backend() -> Result<InputBackend, String> {
    act::config::get_default_backend()
}

#[tauri::command]
pub fn act_set_default_backend(backend: InputBackend) -> Result<(), String> {
    act::config::set_default_backend(backend)
}

#[tauri::command]
pub fn act_get_next_index() -> Result<u32, String> {
    act::config::get_next_available_index()
}

#[tauri::command]
pub fn act_create(action: ActionData, name: Option<String>) -> Result<u32, String> {
    act::config::create_action(action, name)
}

#[tauri::command]
pub fn act_update(index: u32, action: ActionData, name: Option<String>) -> Result<(), String> {
    act::config::update_action(index, action, name)
}

#[tauri::command]
pub fn act_delete(index: u32) -> Result<(), String> {
    act::config::delete_action(index)
}

#[tauri::command]
pub fn act_save_config(
    path: String,
    default_backend: InputBackend,
    actions: ActionList,
) -> Result<(), String> {
    act::config::save_config(&path, default_backend, &actions)
}

#[tauri::command]
pub fn act_execute_action(action_idx: u32) -> Result<(), String> {
    let default_backend = act::config::get_default_backend()
        .unwrap_or(InputBackend::Win32);
    act::executor::execute_action(action_idx, default_backend)
}

#[tauri::command]
pub fn act_execute_action_with_params(
    action_idx: u32,
    params: HashMap<String, Value>,
) -> Result<(), String> {
    let default_backend = act::config::get_default_backend()
        .unwrap_or(InputBackend::Win32);
    act::executor::execute_action_with_params(action_idx, params, default_backend)
}

#[tauri::command]
pub fn act_undo() -> Result<(), String> {
    act::history::undo()
}

#[tauri::command]
pub fn act_redo() -> Result<(), String> {
    act::history::redo()
}

#[tauri::command]
pub fn act_get_history_status() -> Result<(usize, usize), String> {
    Ok(act::history::get_history_status())
}

#[tauri::command]
pub fn act_clear_history() -> Result<(), String> {
    act::history::clear_history()
}

#[tauri::command]
pub fn act_discard_all() -> Result<(), String> {
    act::history::discard()
}
