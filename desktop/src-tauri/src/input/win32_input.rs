//! Windows API 输入模拟 - 直接向目标窗口发送输入
//!
//! 与发送到前台窗口的 enigo 不同，此模块使用
//! PostMessage 直接向特定窗口句柄发送输入。

use windows::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_TYPE, KEYBDINPUT, KEYBD_EVENT_FLAGS, VIRTUAL_KEY,
};
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;
use windows::Win32::UI::WindowsAndMessaging::{
    PostMessageW, WM_CHAR, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN,
    WM_MBUTTONUP, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP,
};

/// 虚拟键码映射到特殊键
pub fn vk_for_key(key: &str) -> Option<u32> {
    match key.to_lowercase().as_str() {
        // 特殊键
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

        // 功能键
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

        // 修饰符键
        "shift" | "rshift" | "lshift" => Some(0x10),
        "control" | "ctrl" | "rcontrol" | "lcontrol" | "rctrl" | "lctrl" => Some(0x11),
        "alt" | "ralt" | "lalt" => Some(0x12),
        "meta" | "super" | "win" => Some(0x5B),

        "capslock" => Some(0x14),

        _ => None,
    }
}

/// 为 WM_KEYDOWN/WM_KEYUP 构建 lparam
/// 位：[31:16] = 扫描码，[15:0] = 重复计数等。
fn build_key_lparam(scan_code: u32, is_keydown: bool, is_extended: bool) -> isize {
    let repeat = 1u32;
    let ext = if is_extended { 0x0100_0000u32 } else { 0u32 };

    if is_keydown {
        (repeat as isize) | ((scan_code as isize) << 16) | (ext as isize)
    } else {
        0xC000_0000_isize | (repeat as isize) | ((scan_code as isize) << 16) | (ext as isize)
    }
}

/// 使用 PostMessage 向窗口发送按键事件
fn post_key_event(hwnd: HWND, vk: u32, scan: u32, is_keydown: bool) -> Result<(), String> {
    let msg = if is_keydown { WM_KEYDOWN } else { WM_KEYUP };
    let lparam = LPARAM(build_key_lparam(scan, is_keydown, false));

    unsafe {
        let _ = PostMessageW(hwnd, msg, WPARAM(vk as usize), lparam);
    }
    Ok(())
}

/// 使用 PostMessage 向窗口发送字符事件
fn post_char_event(hwnd: HWND, c: char) -> Result<(), String> {
    unsafe {
        let _ = PostMessageW(hwnd, WM_CHAR, WPARAM(c as usize), LPARAM(1));
    }
    Ok(())
}

/// 使用 PostMessage 向窗口发送鼠标点击事件
fn post_mouse_click(
    hwnd: HWND,
    x: i32,
    y: i32,
    is_keydown: bool,
    button: u32,
) -> Result<(), String> {
    let msg = match button {
        0 => {
            if is_keydown {
                WM_LBUTTONDOWN
            } else {
                WM_LBUTTONUP
            }
        }
        1 => {
            if is_keydown {
                WM_RBUTTONDOWN
            } else {
                WM_RBUTTONUP
            }
        }
        2 => {
            if is_keydown {
                WM_MBUTTONDOWN
            } else {
                WM_MBUTTONUP
            }
        }
        _ => return Err(format!("Unknown mouse button: {}", button)),
    };

    let lparam = LPARAM((((y as u32) << 16) | (x as u32)) as isize);

    unsafe {
        let _ = PostMessageW(hwnd, msg, WPARAM(button as usize), lparam);
    }
    Ok(())
}

/// 向窗口发送具有指定方向的按键
pub fn send_key(hwnd: isize, key: &str, direction: &str) -> Result<(), String> {
    let hwnd = HWND(hwnd as *mut std::ffi::c_void);

    if hwnd.0.is_null() {
        return Err("Invalid window handle (null HWND)".to_string());
    }

    match direction.to_lowercase().as_str() {
        "press" | "down" => {
            // 首先尝试虚拟键
            if let Some(vk) = vk_for_key(key) {
                // 为简单起见使用扫描码 0 - 许多应用程序不需要确切的扫描码
                post_key_event(hwnd, vk, 0, true)?;
            } else if key.len() == 1 {
                // 单字符 - 作为字符消息发送
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
            // 同时发送按下和释放
            if let Some(vk) = vk_for_key(key) {
                post_key_event(hwnd, vk, 0, true)?;
                std::thread::sleep(std::time::Duration::from_millis(10));
                post_key_event(hwnd, vk, 0, false)?;
            } else if key.len() == 1 {
                let c = key.chars().next().unwrap();
                post_char_event(hwnd, c)?;
                std::thread::sleep(std::time::Duration::from_millis(10));
                // 对于字符点击，我们不发送单独的释放
                // 字符通常作为单个事件发送
            } else {
                return Err(format!("Unknown key: {}", key));
            }
        }
        _ => return Err(format!("Unknown direction: {}", direction)),
    }

    Ok(())
}

/// 逐字符向窗口发送文本
pub fn send_text(hwnd: isize, text: &str) -> Result<(), String> {
    let hwnd = HWND(hwnd as *mut std::ffi::c_void);

    if hwnd.0.is_null() {
        return Err("Invalid window handle (null HWND)".to_string());
    }

    for c in text.chars() {
        post_char_event(hwnd, c)?;
        // 字符之间的小延迟以防止消息丢失
        std::thread::sleep(std::time::Duration::from_millis(2));
    }

    Ok(())
}

/// 从 act::types 导入 MouseButton（统一枚举定义）
use crate::act::types::MouseButton;

/// 向窗口发送指定坐标的鼠标点击
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

/// 将鼠标光标移动到指定的屏幕坐标
pub fn send_mouse_move(x: i32, y: i32) -> Result<(), String> {
    unsafe { SetCursorPos(x, y) }.map_err(|e| format!("Failed to move mouse: {}", e))
}

/// 向窗口发送鼠标滚动事件
///
/// # 参数
/// * `hwnd` - 目标窗口句柄
/// * `delta` - 滚动量（正=上，负=下）。Windows 默认每个"点击"是 120
pub fn send_mouse_scroll(hwnd: isize, delta: i32) -> Result<(), String> {
    let hwnd = HWND(hwnd as *mut std::ffi::c_void);

    if hwnd.0.is_null() {
        return Err("Invalid window handle (null HWND)".to_string());
    }

    // 获取当前鼠标位置用于 lparam
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
        GetCursorPos(&mut point).map_err(|e| format!("Failed to get cursor position: {}", e))?;
    }

    let lparam = LPARAM((((point.y as u32) << 16) | (point.x as u32)) as isize);
    let wparam = WPARAM((delta as u32) as usize);

    unsafe {
        let _ = PostMessageW(hwnd, WM_MOUSEWHEEL, wparam, lparam);
    }

    Ok(())
}

/// 使用 SendInput 发送一系列按键（硬件级模拟）
///
/// # 参数
/// * `keys` - (key_name, is_press) 元组的数组。is_press=true 表示按下，false 表示释放
///
/// 此函数使用 SendInput 原子地发送多个按键事件，
/// 这对于像 ctrl+c 这样的组合键更可靠。
pub fn send_key_sequence(keys: &[(&str, bool)]) -> Result<(), String> {
    let mut inputs: Vec<INPUT> = Vec::with_capacity(keys.len());

    for &(key, is_press) in keys {
        if let Some(vk) = vk_for_key(key) {
            let flags = if is_press {
                KEYBD_EVENT_FLAGS(0) // 按下
            } else {
                KEYBD_EVENT_FLAGS(2) // KEYEVENTF_KEYUP
            };
            let ki = KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk as u16),
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            };
            inputs.push(INPUT {
                r#type: INPUT_TYPE(1), // INPUT_KEYBOARD
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 { ki },
            });
        }
    }

    if inputs.is_empty() {
        return Ok(());
    }

    unsafe {
        let result = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        if result as usize != inputs.len() {
            return Err(format!(
                "SendInput failed, sent {} of {} events",
                result,
                inputs.len()
            ));
        }
    }

    Ok(())
}
