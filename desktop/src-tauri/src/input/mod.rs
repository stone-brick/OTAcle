//! 输入模块 - 处理窗口激活和输入模拟

pub mod enigo;
pub mod find_window;
pub mod win32_input;
pub mod window;

use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetWindowRect};

pub use enigo::{send_key, send_text};
pub use find_window::{
    find_window, find_window_by_class_name, find_window_by_hwnd, find_windows_by_exe,
    find_windows_by_pid, find_windows_by_title, find_windows_by_title_contains, get_window_info,
    list_windows, parse_window_spec,
};
pub use win32_input::{
    send_key as send_key_to_window, send_key_sequence, send_mouse_click, send_mouse_move,
    send_text as send_text_to_window,
};
pub use window::activate_window;

/// 获取屏幕上当前鼠标位置
pub fn get_mouse_position() -> Result<(i32, i32), String> {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point).map_err(|e| format!("Failed to get mouse position: {}", e))?;
        Ok((point.x, point.y))
    }
}

/// 将窗口相对坐标转换为屏幕绝对坐标
pub fn client_to_screen(hwnd: isize, client_x: i32, client_y: i32) -> Result<(i32, i32), String> {
    unsafe {
        let hwnd = HWND(hwnd as *mut std::ffi::c_void);
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).map_err(|e| format!("GetWindowRect failed: {}", e))?;
        // GetWindowRect 返回的是窗口在屏幕上的矩形，左上角就是客户区的屏幕起始位置
        Ok((rect.left + client_x, rect.top + client_y))
    }
}

/// 从 (x0, y0) 到 (x1, y1) 在 duration_ms 时间内平滑移动鼠标
///
/// 如果 duration_ms 为 0，则瞬间移动。
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
