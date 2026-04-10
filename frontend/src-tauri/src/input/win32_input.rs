//! Windows API input simulation - sends input directly to target window
//!
//! Unlike enigo which sends to the foreground window, this module uses
//! PostMessage to send input directly to a specific window handle.

use windows::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    PostMessageW, WM_CHAR, WM_KEYDOWN, WM_KEYUP,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_RBUTTONDOWN, WM_RBUTTONUP,
};
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

/// Virtual key code mapping for special keys
pub fn vk_for_key(key: &str) -> Option<u32> {
    match key.to_lowercase().as_str() {
        // Special keys
        "return" | "enter" => Some(0x0D),
        "space" => Some(0x20),
        "tab" => Some(0x09),
        "escape" | "esc" => Some(0x1B),
        "backspace" => Some(0x08),
        "delete" | "del" => Some(0x2E),
        "up" | "uparrow" => Some(0x26),
        "down" | "downarrow" => Some(0x28),
        "left" | "leftarrow" => Some(0x25),
        "right" | "rightarrow" => Some(0x27),
        "home" => Some(0x24),
        "end" => Some(0x23),
        "pageup" => Some(0x21),
        "pagedown" => Some(0x22),

        // Function keys
        "f1" => Some(0x70),
        "f2" => Some(0x71),
        "f3" => Some(0x72),
        "f4" => Some(0x73),
        "f5" => Some(0x74),
        "f6" => Some(0x75),
        "f7" => Some(0x76),
        "f8" => Some(0x77),
        "f9" => Some(0x78),
        "f10" => Some(0x79),
        "f11" => Some(0x7A),
        "f12" => Some(0x7B),

        // Modifier keys
        "shift" | "rshift" | "lshift" => Some(0x10),
        "control" | "ctrl" | "rcontrol" | "lcontrol" | "rctrl" | "lctrl" => Some(0x11),
        "alt" | "ralt" | "lalt" => Some(0x12),
        "meta" | "super" | "win" => Some(0x5B),

        "capslock" => Some(0x14),

        _ => None,
    }
}

/// Build lparam for WM_KEYDOWN/WM_KEYUP
/// Bits: [31:16] = scan code, [15:0] = repeat count, etc.
fn build_key_lparam(scan_code: u32, is_keydown: bool, is_extended: bool) -> isize {
    let repeat = 1u32;
    let ext = if is_extended { 0x0100_0000u32 } else { 0u32 };

    if is_keydown {
        (repeat as isize) | ((scan_code as isize) << 16) | (ext as isize)
    } else {
        0xC000_0000_isize | (repeat as isize) | ((scan_code as isize) << 16) | (ext as isize)
    }
}

/// Send a key event to a window using PostMessage
fn post_key_event(hwnd: HWND, vk: u32, scan: u32, is_keydown: bool) -> Result<(), String> {
    let msg = if is_keydown { WM_KEYDOWN } else { WM_KEYUP };
    let lparam = LPARAM(build_key_lparam(scan, is_keydown, false));

    unsafe {
        let _ = PostMessageW(hwnd, msg, WPARAM(vk as usize), lparam);
    }
    Ok(())
}

/// Send a character event to a window using PostMessage
fn post_char_event(hwnd: HWND, c: char) -> Result<(), String> {
    unsafe {
        let _ = PostMessageW(hwnd, WM_CHAR, WPARAM(c as usize), LPARAM(1));
    }
    Ok(())
}

/// Send a mouse click event to a window using PostMessage
fn post_mouse_click(hwnd: HWND, x: i32, y: i32, is_keydown: bool, button: u32) -> Result<(), String> {
    let msg = match button {
        0 => if is_keydown { WM_LBUTTONDOWN } else { WM_LBUTTONUP },
        1 => if is_keydown { WM_RBUTTONDOWN } else { WM_RBUTTONUP },
        2 => if is_keydown { WM_MBUTTONDOWN } else { WM_MBUTTONUP },
        _ => return Err(format!("Unknown mouse button: {}", button)),
    };

    let lparam = LPARAM((((y as u32) << 16) | (x as u32)) as isize);

    unsafe {
        let _ = PostMessageW(hwnd, msg, WPARAM(button as usize), lparam);
    }
    Ok(())
}

/// Send a key to a window with specified direction
pub fn send_key(hwnd: isize, key: &str, direction: &str) -> Result<(), String> {
    let hwnd = HWND(hwnd as *mut std::ffi::c_void);

    if hwnd.0.is_null() {
        return Err("Invalid window handle (null HWND)".to_string());
    }

    match direction.to_lowercase().as_str() {
        "press" | "down" => {
            // Try virtual key first
            if let Some(vk) = vk_for_key(key) {
                // Use scan code 0 for simplicity - many apps don't need the exact scan code
                post_key_event(hwnd, vk, 0, true)?;
            } else if key.len() == 1 {
                // Single character - send as character message
                let c = key.chars().next().unwrap();
                post_char_event(hwnd, c)?;
            } else {
                return Err(format!("Unknown key: {}", key));
            }
        }
        "release" | "up" => {
            if let Some(vk) = vk_for_key(key) {
                post_key_event(hwnd, vk, 0, false)?;
            } else if key.len() == 1 {
                let c = key.chars().next().unwrap();
                post_char_event(hwnd, c)?;
            } else {
                return Err(format!("Unknown key: {}", key));
            }
        }
        "click" => {
            // Send both down and up
            if let Some(vk) = vk_for_key(key) {
                post_key_event(hwnd, vk, 0, true)?;
                std::thread::sleep(std::time::Duration::from_millis(10));
                post_key_event(hwnd, vk, 0, false)?;
            } else if key.len() == 1 {
                let c = key.chars().next().unwrap();
                post_char_event(hwnd, c)?;
                std::thread::sleep(std::time::Duration::from_millis(10));
                // For click on character, we don't send a separate release
                // Characters are typically sent as single events
            } else {
                return Err(format!("Unknown key: {}", key));
            }
        }
        _ => return Err(format!("Unknown direction: {}", direction)),
    }

    Ok(())
}

/// Send text to a window character by character
pub fn send_text(hwnd: isize, text: &str) -> Result<(), String> {
    let hwnd = HWND(hwnd as *mut std::ffi::c_void);

    if hwnd.0.is_null() {
        return Err("Invalid window handle (null HWND)".to_string());
    }

    for c in text.chars() {
        post_char_event(hwnd, c)?;
        // Small delay between characters to prevent message loss
        std::thread::sleep(std::time::Duration::from_millis(2));
    }

    Ok(())
}

/// Mouse button enum for win32 module
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Send a mouse click to a window at specified coordinates
pub fn send_mouse_click(hwnd: isize, x: i32, y: i32, button: MouseButton) -> Result<(), String> {
    let hwnd = HWND(hwnd as *mut std::ffi::c_void);

    if hwnd.0.is_null() {
        return Err("Invalid window handle (null HWND)".to_string());
    }

    let btn = match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
    };

    post_mouse_click(hwnd, x, y, true, btn)?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    post_mouse_click(hwnd, x, y, false, btn)?;

    Ok(())
}

/// Move mouse cursor to specified screen coordinates
pub fn send_mouse_move(x: i32, y: i32) -> Result<(), String> {
    unsafe {
        SetCursorPos(x, y)
    }.map_err(|e| format!("Failed to move mouse: {}", e))
}
