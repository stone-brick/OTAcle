use crate::input;
use crate::state::get_hwnd_from_spec;
use crate::input::find_window::WindowInfo;
use windows::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetForegroundWindow, WM_KEYDOWN, WM_KEYUP, WM_MOUSEMOVE, PostMessageW};

#[tauri::command]
pub fn activate_window(hwnd: i64) -> Result<bool, String> {
    input::activate_window(hwnd as isize)
}

#[tauri::command]
pub fn get_foreground_window() -> Result<i64, String> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0.is_null() {
        Err("No foreground window found".to_string())
    } else {
        Ok(hwnd.0 as i64)
    }
}

#[tauri::command]
pub fn get_window_info(hwnd: i64) -> Result<WindowInfo, String> {
    input::get_window_info(hwnd as isize)
        .ok_or_else(|| "Window not found".to_string())
}

#[tauri::command]
pub fn list_windows() -> Result<Vec<WindowInfo>, String> {
    Ok(input::list_windows())
}

#[tauri::command]
pub fn send_key(key: String, direction: String, window: Option<String>) -> Result<(), String> {
    if let Some(ref spec) = window {
        let hwnd = get_hwnd_from_spec(spec)?;
        input::activate_window(hwnd)?;
    }
    input::send_key(&key, &direction)
}

#[tauri::command]
pub fn send_text(text: String, window: Option<String>) -> Result<(), String> {
    if let Some(ref spec) = window {
        let hwnd = get_hwnd_from_spec(spec)?;
        input::activate_window(hwnd)?;
    }
    input::send_text(&text)
}

#[tauri::command]
pub fn get_mouse_position() -> Result<(i32, i32), String> {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point).map_err(|e| format!("Failed to get mouse position: {}", e))?;
        Ok((point.x, point.y))
    }
}

#[tauri::command]
pub fn send_mouse_click(
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
            input::send_mouse_click(hwnd as isize, x, y, btn)?;
            Ok(())
        }
    }
}

#[tauri::command]
pub fn send_mouse_move(hwnd: i64, x: i32, y: i32, backend: Option<String>) -> Result<(), String> {
    match backend.as_deref() {
        Some("win32") => {
            let hwnd = HWND(hwnd as *mut std::ffi::c_void);
            unsafe {
                let lparam = LPARAM((((y as u32) << 16) | (x as u32)) as isize);
                let _ = PostMessageW(
                    hwnd,
                    WM_MOUSEMOVE,
                    WPARAM(0),
                    lparam,
                );
            }
            Ok(())
        }
        _ => {
            input::send_mouse_move(x, y)
        }
    }
}

#[tauri::command]
pub fn send_combination_key(
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
            let hwnd = HWND(hwnd as *mut std::ffi::c_void);

            for key in &keys {
                if let Some(vk) = input::win32_input::vk_for_key(key) {
                    unsafe {
                        let _ = PostMessageW(
                            hwnd, WM_KEYDOWN, WPARAM(vk as usize), LPARAM(1),
                        );
                    }
                }
            }
            for key in keys.iter().rev() {
                if let Some(vk) = input::win32_input::vk_for_key(key) {
                    unsafe {
                        let _ = PostMessageW(
                            hwnd, WM_KEYUP, WPARAM(vk as usize), LPARAM(0xC0000001),
                        );
                    }
                }
            }
            Ok(())
        }
        _ => {
            if let Some(spec) = window {
                let hwnd = get_hwnd_from_spec(&spec)?;
                input::activate_window(hwnd)?;
            }
            if keys.len() == 2 {
                let combo = format!("{}+{}", keys[0], keys[1]);
                input::send_key(&combo, "click")
            } else {
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

#[tauri::command]
pub fn activate_window_by_title(title: String) -> Result<bool, String> {
    let search = input::parse_window_spec(&title);

    let hwnd = if search.title.is_none()
        && search.class_name.is_none()
        && search.hwnd.is_none()
        && search.pid.is_none()
        && search.exe_name.is_none()
    {
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
