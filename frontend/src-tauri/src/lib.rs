mod input;

use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

fn char_needs_shift(c: char) -> bool {
    c.is_uppercase() || "!@#$%^&*()_+{}|:\"<>?".contains(c)
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

#[tauri::command]
fn activate_window(hwnd: i64) -> Result<bool, String> {
    input::activate_window(hwnd as isize)
}

#[tauri::command]
fn send_key(key: String, direction: String, window: Option<String>) -> Result<(), String> {
    // If window spec is provided, activate it first
    if let Some(ref spec) = window {
        let hwnd = get_hwnd_from_spec(spec)?;
        input::activate_window(hwnd)?;
    }

    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

    let dir = match direction.to_lowercase().as_str() {
        "press" => Direction::Press,
        "release" => Direction::Release,
        "click" | _ => Direction::Click,
    };

    let enigo_key = match key.to_lowercase().as_str() {
        "return" | "enter" => Key::Return,
        "space" => Key::Space,
        "tab" => Key::Tab,
        "escape" | "esc" => Key::Escape,
        "backspace" => Key::Backspace,
        "delete" => Key::Delete,
        "up" => Key::UpArrow,
        "down" => Key::DownArrow,
        "left" => Key::LeftArrow,
        "right" => Key::RightArrow,
        "home" => Key::Home,
        "end" => Key::End,
        "pageup" => Key::PageUp,
        "pagedown" => Key::PageDown,
        "shift" => Key::Shift,
        "ctrl" | "control" => Key::Control,
        "alt" => Key::Alt,
        "capslock" => Key::CapsLock,
        _ => {
            if key.len() == 1 {
                Key::Unicode(key.chars().next().unwrap())
            } else {
                return Err(format!("Unknown key: {}", key));
            }
        }
    };

    enigo.key(enigo_key, dir).map_err(|e| e.to_string())
}

#[tauri::command]
fn send_text(text: String, window: Option<String>) -> Result<(), String> {
    // If window spec is provided, activate it first
    if let Some(ref spec) = window {
        let hwnd = get_hwnd_from_spec(spec)?;
        input::activate_window(hwnd)?;
    }

    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

    for c in text.chars() {
        if char_needs_shift(c) {
            enigo.key(Key::Shift, Direction::Press).map_err(|e| e.to_string())?;
            enigo.key(Key::Unicode(c.to_ascii_lowercase()), Direction::Click)
                .map_err(|e| e.to_string())?;
            enigo.key(Key::Shift, Direction::Release).map_err(|e| e.to_string())?;
        } else {
            enigo.key(Key::Unicode(c), Direction::Click)
                .map_err(|e| e.to_string())?;
        }
        // Small delay between characters to let the target app process input
        std::thread::sleep(std::time::Duration::from_millis(2));
    }

    Ok(())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
