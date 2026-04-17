use crate::act;
use act::types::InputBackend;

#[tauri::command]
pub fn act_set_execution_backend(backend: String) -> Result<(), String> {
    let backend = match backend.as_str() {
        "win32" => InputBackend::Win32,
        "enigo" => InputBackend::Enigo,
        _ => return Err(format!("Unknown backend: {}", backend)),
    };
    act::config::set_execution_backend(backend)
}

#[tauri::command]
pub fn act_get_execution_backend() -> Result<Option<InputBackend>, String> {
    Ok(act::config::get_execution_backend())
}
