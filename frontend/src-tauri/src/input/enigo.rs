//! 使用 enigo 进行输入模拟 - 全局单例实现

use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// 全局 Enigo 实例 - 创建一次并重复使用
static ENIGO: Lazy<Mutex<Enigo>> = Lazy::new(|| {
    Mutex::new(
        Enigo::new(&Settings::default())
            .expect("Failed to create Enigo instance")
    )
});

/// 使用 enigo 的 text() API 发送字符串
pub fn send_text(text: &str) -> Result<(), String> {
    let mut enigo = ENIGO.lock()
        .map_err(|_| "Failed to lock Enigo".to_string())?;

    enigo
        .text(text)
        .map_err(|e| format!("Failed to send text: {:?}", e))?;

    Ok(())
}

/// 发送具有指定方向的单个按键
///
/// direction: "press"、"release" 或 "click"
pub fn send_key(key_str: &str, direction: &str) -> Result<(), String> {
    let mut enigo = ENIGO.lock()
        .map_err(|_| "Failed to lock Enigo".to_string())?;

    let key = parse_key(key_str)?;
    let dir = parse_direction(direction)?;

    enigo
        .key(key, dir)
        .map_err(|e| format!("Failed to send key: {:?}", e))?;

    Ok(())
}

/// 将按键字符串解析为 enigo::Key
fn parse_key(key_str: &str) -> Result<Key, String> {
    match key_str.to_lowercase().as_str() {
        // 特殊键
        "return" | "enter" => Ok(Key::Return),
        "space" => Ok(Key::Space),
        "tab" => Ok(Key::Tab),
        "escape" | "esc" => Ok(Key::Escape),
        "backspace" => Ok(Key::Backspace),
        "delete" | "del" => Ok(Key::Delete),
        "up" | "uparrow" => Ok(Key::UpArrow),
        "down" | "downarrow" => Ok(Key::DownArrow),
        "left" | "leftarrow" => Ok(Key::LeftArrow),
        "right" | "rightarrow" => Ok(Key::RightArrow),
        "home" => Ok(Key::Home),
        "end" => Ok(Key::End),
        "pageup" => Ok(Key::PageUp),
        "pagedown" => Ok(Key::PageDown),

        // 功能键
        "f1" => Ok(Key::F1),
        "f2" => Ok(Key::F2),
        "f3" => Ok(Key::F3),
        "f4" => Ok(Key::F4),
        "f5" => Ok(Key::F5),
        "f6" => Ok(Key::F6),
        "f7" => Ok(Key::F7),
        "f8" => Ok(Key::F8),
        "f9" => Ok(Key::F9),
        "f10" => Ok(Key::F10),
        "f11" => Ok(Key::F11),
        "f12" => Ok(Key::F12),

        // 修饰符键
        "shift" | "rshift" | "lshift" => Ok(Key::Shift),
        "control" | "ctrl" | "rcontrol" | "lcontrol" | "rctrl" | "lctrl" => Ok(Key::Control),
        "alt" | "ralt" | "lalt" => Ok(Key::Alt),
        "meta" | "super" | "win" => Ok(Key::Meta),

        // 其他
        "capslock" => Ok(Key::CapsLock),

        // 单字符 - 使用 Unicode
        _ if key_str.len() == 1 => {
            let c = key_str.chars().next().unwrap();
            Ok(Key::Unicode(c))
        }

        _ => Err(format!("Unknown key: {}", key_str)),
    }
}

/// 将方向字符串解析为 enigo::Direction
fn parse_direction(dir_str: &str) -> Result<Direction, String> {
    match dir_str.to_lowercase().as_str() {
        "press" => Ok(Press),
        "release" => Ok(Release),
        "click" => Ok(Click),
        _ => Err(format!("Unknown direction: {}", dir_str)),
    }
}

// 重新导出 Direction 以便调用者可以使用
pub use enigo::Direction;
