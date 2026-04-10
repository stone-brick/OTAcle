mod action;
mod input;

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetCursorPos};

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
fn execute_action(action_id: u32, window: Option<String>, backend: Option<String>) -> Result<(), String> {
    // Get default backend from config, fallback to Win32 if not loaded
    let default_backend = action::config::get_default_backend()
        .unwrap_or(action::types::InputBackend::Win32);
    action::executor::execute_action(action_id, window, backend, default_backend)
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
                        use windows::Win32::UI::WindowsAndMessaging::{WM_KEYDOWN, WM_KEYUP};
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
        .invoke_handler(tauri::generate_handler![
            activate_window,
            send_key,
            send_text,
            find_window_by_title,
            activate_window_by_title,
            load_action_config,
            execute_action,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
