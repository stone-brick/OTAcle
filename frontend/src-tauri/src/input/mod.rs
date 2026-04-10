//! Input module - handles window activation and input simulation

pub mod enigo;
pub mod find_window;
pub mod window;

pub use enigo::{send_key, send_text};
pub use find_window::{find_window, find_all_windows, parse_window_spec};
pub use window::activate_window;
