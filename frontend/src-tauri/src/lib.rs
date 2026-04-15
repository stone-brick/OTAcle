mod action;
mod input;
mod zmq_pull;

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetCursorPos};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::Emitter;
use zmq_pull::{ZmqCommand, ZmqPuller};

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

/// Helper to get HWND from a window spec string
fn get_hwnd_from_spec(spec: &str) -> Result<isize, String> {
    let search = input::parse_window_spec(spec);

    // Empty spec means foreground window
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

/// Start ZMQ receiver to receive commands from Python
#[tauri::command]
fn zmq_start(
    addr: String,
    state: tauri::State<'_, ZmqState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Stop existing receiver if any
    {
        let mut running_guard = state.running.lock().map_err(|_| "Lock failed")?;
        if let Some(running) = running_guard.take() {
            running.store(false, Ordering::SeqCst);
        }
    }

    // Store new address
    {
        let mut addr_guard = state.address.lock().map_err(|_| "Lock failed")?;
        *addr_guard = addr.clone();
    }

    // Create new receiver
    let puller = ZmqPuller::new(&addr)?;

    // Create running flag
    let running = Arc::new(AtomicBool::new(true));

    // Clone for storage
    let running_for_state = running.clone();
    let running_for_thread = running.clone();

    // Store running flag
    {
        let mut running_guard = state.running.lock().map_err(|_| "Lock failed")?;
        *running_guard = Some(running_for_state);
    }

    // Clone app handle for the callback
    let app_for_callback = app.clone();

    // Start listening in a separate thread
    puller.start(running_for_thread, move |cmd: ZmqCommand| {
        // Execute actions based on the execute vector
        let default_backend = action::config::get_default_backend()
            .unwrap_or(action::types::InputBackend::Win32);

        let result = action::executor::execute_actions(
            cmd.execute,
            cmd.params,
            default_backend,
        );

        match result {
            Ok(()) => {
                let msg = "[ZMQ] Actions executed successfully".to_string();
                let _ = app_for_callback.emit("zmq:log", msg);
            }
            Err(e) => {
                let msg = format!("[ZMQ] Error: {}", e);
                let _ = app_for_callback.emit("zmq:error", msg);
            }
        }
    })?;

    Ok(())
}

/// Stop ZMQ receiver
#[tauri::command]
fn zmq_stop(state: tauri::State<'_, ZmqState>) -> Result<(), String> {
    let mut running_guard = state.running.lock().map_err(|_| "Lock failed")?;
    if let Some(running) = running_guard.take() {
        running.store(false, Ordering::SeqCst);
    }
    Ok(())
}

/// Get ZMQ connection status
#[tauri::command]
fn zmq_status(state: tauri::State<'_, ZmqState>) -> Result<(bool, String), String> {
    let running_guard = state.running.lock().map_err(|_| "Lock failed")?;
    let addr_guard = state.address.lock().map_err(|_| "Lock failed")?;

    let connected = running_guard.is_some();
    let address = addr_guard.clone();

    Ok((connected, address))
}

#[tauri::command]
fn activate_window(hwnd: i64) -> Result<bool, String> {
    input::activate_window(hwnd as isize)
}

#[tauri::command]
fn send_key(key: String, direction: String, window: Option<String>) -> Result<(), String> {
    if let Some(ref spec) = window {
        let hwnd = get_hwnd_from_spec(spec)?;
        input::activate_window(hwnd)?;
    }
    input::send_key(&key, &direction)
}

#[tauri::command]
fn send_text(text: String, window: Option<String>) -> Result<(), String> {
    if let Some(ref spec) = window {
        let hwnd = get_hwnd_from_spec(spec)?;
        input::activate_window(hwnd)?;
    }
    input::send_text(&text)
}

#[tauri::command]
fn find_window_by_title(title: String) -> Result<Option<i64>, String> {
    let search = input::parse_window_spec(&title);

    // Empty spec means foreground window - use GetForegroundWindow directly
    if search.title.is_none()
        && search.class_name.is_none()
        && search.hwnd.is_none()
        && search.pid.is_none()
        && search.exe_name.is_none()
    {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.0.is_null() {
            return Ok(None);
        }
        return Ok(Some(hwnd.0 as i64));
    }

    Ok(input::find_window(&search).map(|h| h as i64))
}

#[tauri::command]
fn activate_window_by_title(title: String) -> Result<bool, String> {
    let search = input::parse_window_spec(&title);

    // Empty spec means foreground window - use GetForegroundWindow directly
    let hwnd = if search.title.is_none()
        && search.class_name.is_none()
        && search.hwnd.is_none()
        && search.pid.is_none()
        && search.exe_name.is_none()
    {
        // Get current foreground window
        let hwnd = unsafe { windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow() };
        if hwnd.0.is_null() {
            return Err("No foreground window found".to_string());
        }
        hwnd.0 as isize
    } else {
        input::find_window(&search).ok_or_else(|| "Window not found".to_string())?
    };

    input::activate_window(hwnd)
}

#[tauri::command]
fn load_action_config(path: String, backend: Option<String>) -> Result<(), String> {
    use action::types::InputBackend;

    let backend = match backend.as_deref() {
        Some("win32") => InputBackend::Win32,
        _ => InputBackend::Enigo,
    };

    action::config::load_config_with_backend(&path, backend)
}

#[tauri::command]
fn execute_action(action_id: u32) -> Result<(), String> {
    // Get default backend from config, fallback to Win32 if not loaded
    let default_backend = action::config::get_default_backend()
        .unwrap_or(action::types::InputBackend::Win32);
    action::executor::execute_action(action_id, default_backend)
}

#[tauri::command]
fn execute_action_with_params(
    action_id: u32,
    params: std::collections::HashMap<String, serde_json::Value>,
) -> Result<(), String> {
    // Get default backend from config, fallback to Win32 if not loaded
    let default_backend = action::config::get_default_backend()
        .unwrap_or(action::types::InputBackend::Win32);
    action::executor::execute_action_with_params(action_id, params, default_backend)
}

#[tauri::command]
fn set_target_window(window: Option<String>) -> Result<(), String> {
    action::config::set_target_window(window)
}

#[tauri::command]
fn get_target_window() -> Result<Option<i64>, String> {
    Ok(action::config::get_target_window().map(|hwnd| hwnd as i64))
}

#[tauri::command]
fn set_execution_backend(backend: String) -> Result<(), String> {
    let backend = match backend.as_str() {
        "win32" => action::types::InputBackend::Win32,
        "enigo" => action::types::InputBackend::Enigo,
        _ => return Err(format!("Unknown backend: {}", backend)),
    };
    action::config::set_execution_backend(backend)
}

#[tauri::command]
fn get_execution_backend() -> Result<action::types::InputBackend, String> {
    Ok(action::config::get_execution_backend())
}

#[tauri::command]
fn list_windows() -> Result<Vec<input::find_window::WindowInfo>, String> {
    Ok(input::list_windows())
}

// ============================================================================
// Dedicated window search commands - explicit API for each search type
// ============================================================================

/// Find first window by title (prefix match)
#[tauri::command]
fn find_first_window_by_title(title: String) -> Result<Option<i64>, String> {
    let results = input::find_windows_by_title(&title);
    Ok(results.into_iter().next().map(|h| h as i64))
}

/// Find all windows by title (prefix match)
#[tauri::command]
fn find_windows_by_title(title: String) -> Result<Vec<i64>, String> {
    Ok(input::find_windows_by_title(&title).into_iter().map(|h| h as i64).collect())
}

/// Find first window by title (contains match)
#[tauri::command]
fn find_first_window_by_title_contains(title: String) -> Result<Option<i64>, String> {
    let results = input::find_windows_by_title_contains(&title);
    Ok(results.into_iter().next().map(|h| h as i64))
}

/// Find all windows by title (contains match)
#[tauri::command]
fn find_windows_by_title_contains(title: String) -> Result<Vec<i64>, String> {
    Ok(input::find_windows_by_title_contains(&title).into_iter().map(|h| h as i64).collect())
}

/// Find window by exact class name
#[tauri::command]
fn find_window_by_class(class_name: String) -> Result<Option<i64>, String> {
    Ok(input::find_window_by_class_name(&class_name).map(|h| h as i64))
}

/// Find all windows for a process ID
#[tauri::command]
fn find_windows_by_pid(pid: u32) -> Result<Vec<i64>, String> {
    Ok(input::find_windows_by_pid(pid).into_iter().map(|h| h as i64).collect())
}

/// Find all windows for an executable name
#[tauri::command]
fn find_windows_by_exe(exe_name: String) -> Result<Vec<i64>, String> {
    Ok(input::find_windows_by_exe(&exe_name).into_iter().map(|h| h as i64).collect())
}

/// Find window by exact HWND
#[tauri::command]
fn find_window_by_hwnd(hwnd: i64) -> Result<Option<i64>, String> {
    Ok(input::find_window_by_hwnd(hwnd as isize).map(|h| h as i64))
}

#[tauri::command]
fn get_window_info(hwnd: i64) -> Result<input::find_window::WindowInfo, String> {
    input::get_window_info(hwnd as isize)
        .ok_or_else(|| "Window not found".to_string())
}

#[tauri::command]
fn get_config() -> Result<action::types::ActionConfig, String> {
    action::config::get_config()
}

#[tauri::command]
fn get_action_list() -> Result<action::types::ActionConfigList, String> {
    action::config::get_action_list()
}

#[tauri::command]
fn get_default_backend() -> Result<action::types::InputBackend, String> {
    action::config::get_default_backend()
}

#[tauri::command]
fn set_default_backend(backend: action::types::InputBackend) -> Result<(), String> {
    action::config::set_default_backend(backend)
}

#[tauri::command]
fn get_next_action_index() -> Result<u32, String> {
    action::config::get_next_available_index()
}

#[tauri::command]
fn create_action(action: action::types::Action, name: Option<String>) -> Result<u32, String> {
    action::config::create_action(action, name)
}

#[tauri::command]
fn update_action(index: u32, action: action::types::Action, name: Option<String>) -> Result<(), String> {
    action::config::update_action(index, action, name)
}

#[tauri::command]
fn delete_action(index: u32) -> Result<(), String> {
    action::config::delete_action(index)
}

#[tauri::command]
fn save_action_config(
    path: String,
    default_backend: action::types::InputBackend,
    actions: action::types::ActionConfigList,
) -> Result<(), String> {
    action::config::save_config(&path, default_backend, &actions)
}

#[tauri::command]
fn undo_action() -> Result<(), String> {
    action::config::undo()
}

#[tauri::command]
fn redo_action() -> Result<(), String> {
    action::config::redo()
}

#[tauri::command]
fn get_history_status() -> Result<(usize, usize), String> {
    Ok(action::config::get_history_status())
}

#[tauri::command]
fn clear_action_history() -> Result<(), String> {
    action::config::clear_history()
}

#[tauri::command]
fn discard_changes() -> Result<(), String> {
    action::config::discard_changes()
}

#[tauri::command]
fn discard_action(index: u32) -> Result<(), String> {
    action::config::discard_action(index)
}

#[tauri::command]
fn get_foreground_window() -> Result<i64, String> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0.is_null() {
        Err("No foreground window found".to_string())
    } else {
        Ok(hwnd.0 as i64)
    }
}

#[tauri::command]
fn send_mouse_click(
    hwnd: i64,
    x: i32,
    y: i32,
    button: String,
    backend: Option<String>,
) -> Result<(), String> {
    let btn = match button.to_lowercase().as_str() {
        "left" => input::win32_input::MouseButton::Left,
        "right" => input::win32_input::MouseButton::Right,
        "middle" => input::win32_input::MouseButton::Middle,
        _ => return Err(format!("Unknown mouse button: {}", button)),
    };

    match backend.as_deref() {
        Some("win32") => {
            input::send_mouse_click(hwnd as isize, x, y, btn)?;
            Ok(())
        }
        _ => {
            // Enigo backend - move mouse and click
            let hwnd_val = hwnd as isize;
            // For enigo, we just use the screen coordinates
            input::enigo::send_text("")?; // Just to include the module
            // Use win32 for mouse click since enigo doesn't have direct window target
            input::send_mouse_click(hwnd_val, x, y, btn)?;
            Ok(())
        }
    }
}

#[tauri::command]
fn send_mouse_move(hwnd: i64, x: i32, y: i32, backend: Option<String>) -> Result<(), String> {
    match backend.as_deref() {
        Some("win32") => {
            // Send mouse move to the window
            let hwnd = windows::Win32::Foundation::HWND(hwnd as *mut std::ffi::c_void);
            unsafe {
                use windows::Win32::Foundation::{LPARAM, WPARAM};
                use windows::Win32::UI::WindowsAndMessaging::WM_MOUSEMOVE;
                let lparam = LPARAM((((y as u32) << 16) | (x as u32)) as isize);
                let _ = windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                    hwnd,
                    WM_MOUSEMOVE,
                    WPARAM(0),
                    lparam,
                );
            }
            Ok(())
        }
        _ => {
            // Use SetCursorPos for global mouse move
            input::send_mouse_move(x, y)
        }
    }
}

#[tauri::command]
fn get_mouse_position() -> Result<(i32, i32), String> {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point).map_err(|e| format!("Failed to get mouse position: {}", e))?;
        Ok((point.x, point.y))
    }
}

#[tauri::command]
fn send_combination_key(
    keys: Vec<String>,
    window: Option<String>,
    backend: Option<String>,
) -> Result<(), String> {
    if keys.is_empty() {
        return Err("No keys provided".to_string());
    }

    let hwnd = if let Some(ref spec) = window {
        Some(get_hwnd_from_spec(spec)?)
    } else {
        None
    };

    match backend.as_deref() {
        Some("win32") => {
            let hwnd = hwnd.ok_or_else(|| "Win32 backend requires window target".to_string())?;
            let hwnd = windows::Win32::Foundation::HWND(hwnd as *mut std::ffi::c_void);

            // Press all keys in sequence
            for key in &keys {
                if let Some(vk) = input::win32_input::vk_for_key(key) {
                    unsafe {
                        use windows::Win32::Foundation::{WPARAM, LPARAM};
                        use windows::Win32::UI::WindowsAndMessaging::WM_KEYDOWN;
                        let _ = windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                            hwnd, WM_KEYDOWN, WPARAM(vk as usize), LPARAM(1),
                        );
                    }
                }
            }
            // Release all keys in reverse
            for key in keys.iter().rev() {
                if let Some(vk) = input::win32_input::vk_for_key(key) {
                    unsafe {
                        use windows::Win32::Foundation::{WPARAM, LPARAM};
                        use windows::Win32::UI::WindowsAndMessaging::WM_KEYUP;
                        let _ = windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                            hwnd, WM_KEYUP, WPARAM(vk as usize), LPARAM(0xC0000001),
                        );
                    }
                }
            }
            Ok(())
        }
        _ => {
            // Enigo backend
            if let Some(spec) = window {
                let hwnd = get_hwnd_from_spec(&spec)?;
                input::activate_window(hwnd)?;
            }
            // For enigo, we use the combination key format "ctrl+c"
            if keys.len() == 2 {
                let combo = format!("{}+{}", keys[0], keys[1]);
                input::send_key(&combo, "click")
            } else {
                // For longer combinations, fall back to sequential press
                for key in &keys {
                    input::send_key(key, "press")?;
                }
                for key in keys.iter().rev() {
                    input::send_key(key, "release")?;
                }
                Ok(())
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ZmqState::default())
        .invoke_handler(tauri::generate_handler![
            activate_window,
            send_key,
            send_text,
            find_window_by_title,
            activate_window_by_title,
            load_action_config,
            get_config,
            get_action_list,
            get_default_backend,
            set_default_backend,
            get_next_action_index,
            create_action,
            update_action,
            delete_action,
            save_action_config,
            execute_action,
            execute_action_with_params,
            set_target_window,
            get_target_window,
            set_execution_backend,
            get_execution_backend,
            undo_action,
            redo_action,
            get_history_status,
            clear_action_history,
            discard_changes,
            discard_action,
            list_windows,
            get_window_info,
            get_foreground_window,
            send_mouse_click,
            send_mouse_move,
            get_mouse_position,
            send_combination_key,
            // Dedicated search commands
            find_first_window_by_title,
            find_windows_by_title,
            find_first_window_by_title_contains,
            find_windows_by_title_contains,
            find_window_by_class,
            find_windows_by_pid,
            find_windows_by_exe,
            find_window_by_hwnd,
            // ZMQ commands
            zmq_start,
            zmq_stop,
            zmq_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
