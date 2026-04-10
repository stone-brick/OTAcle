//! Input simulation using enigo - global singleton implementation

use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Global Enigo instance - created once and reused
static ENIGO: Lazy<Mutex<Enigo>> = Lazy::new(|| {
    Mutex::new(
        Enigo::new(&Settings::default())
            .expect("Failed to create Enigo instance")
    )
});

/// Sends a string of text using enigo's text() API
pub fn send_text(text: &str) -> Result<(), String> {
    let mut enigo = ENIGO.lock()
        .map_err(|_| "Failed to lock Enigo".to_string())?;

    enigo
        .text(text)
        .map_err(|e| format!("Failed to send text: {:?}", e))?;

    Ok(())
}

/// Sends a single key with specified direction
///
/// direction: "press", "release", or "click"
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

/// Parses a key string to enigo::Key
fn parse_key(key_str: &str) -> Result<Key, String> {
    match key_str.to_lowercase().as_str() {
        // Special keys
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

        // Function keys
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

        // Modifier keys
        "shift" | "rshift" | "lshift" => Ok(Key::Shift),
        "control" | "ctrl" | "rcontrol" | "lcontrol" | "rctrl" | "lctrl" => Ok(Key::Control),
        "alt" | "ralt" | "lalt" => Ok(Key::Alt),
        "meta" | "super" | "win" => Ok(Key::Meta),

        // Other
        "capslock" => Ok(Key::CapsLock),

        // Single character - use Unicode
        _ if key_str.len() == 1 => {
            let c = key_str.chars().next().unwrap();
            Ok(Key::Unicode(c))
        }

        _ => Err(format!("Unknown key: {}", key_str)),
    }
}

/// Parses a direction string to enigo::Direction
fn parse_direction(dir_str: &str) -> Result<Direction, String> {
    match dir_str.to_lowercase().as_str() {
        "press" => Ok(Press),
        "release" => Ok(Release),
        "click" => Ok(Click),
        _ => Err(format!("Unknown direction: {}", dir_str)),
    }
}

// Re-export Direction so callers can use it
pub use enigo::Direction;
