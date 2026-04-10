//! Input module - handles window activation and input simulation

pub mod enigo;
pub mod find_window;
pub mod window;
pub mod win32_input;

pub use enigo::{send_key, send_text};
pub use find_window::{
    find_window, parse_window_spec, list_windows, get_window_info,
    find_windows_by_title, find_windows_by_title_contains,
    find_window_by_class_name, find_windows_by_pid, find_windows_by_exe,
    find_window_by_hwnd,
};
pub use window::activate_window;
pub use win32_input::{send_key as send_key_to_window, send_text as send_text_to_window, send_mouse_click, send_mouse_move};
