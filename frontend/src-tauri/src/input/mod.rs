//! Input module - handles window activation and input simulation

pub mod enigo;
pub mod find_window;
pub mod window;
pub mod win32_input;

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

pub use enigo::{send_key, send_text};
pub use find_window::{
    find_window, parse_window_spec, list_windows, get_window_info,
    find_windows_by_title, find_windows_by_title_contains,
    find_window_by_class_name, find_windows_by_pid, find_windows_by_exe,
    find_window_by_hwnd,
};
pub use window::activate_window;
pub use win32_input::{send_key as send_key_to_window, send_text as send_text_to_window, send_mouse_click, send_mouse_move, send_key_sequence};

/// Get current mouse position on screen
pub fn get_mouse_position() -> Result<(i32, i32), String> {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point).map_err(|e| format!("Failed to get mouse position: {}", e))?;
        Ok((point.x, point.y))
    }
}

/// Smooth mouse move from (x0, y0) to (x1, y1) over duration_ms
///
/// If duration_ms is 0, moves instantly.
pub fn smooth_move(x0: i32, y0: i32, x1: i32, y1: i32, duration_ms: u64) -> Result<(), String> {
    if duration_ms == 0 {
        return send_mouse_move(x1, y1);
    }

    // Calculate number of steps (at least 60 steps for smoothness)
    let steps = 60.max((duration_ms as i32) / 16);
    let sleep_per_step_ms = duration_ms as f64 / steps as f64;

    for step in 1..=steps {
        let t = step as f64 / steps as f64;
        let x = (x0 as f64 + (x1 as f64 - x0 as f64) * t) as i32;
        let y = (y0 as f64 + (y1 as f64 - y0 as f64) * t) as i32;
        send_mouse_move(x, y)?;
        std::thread::sleep(std::time::Duration::from_millis(sleep_per_step_ms as u64));
    }

    Ok(())
}
