//! Action module - handles action configuration parsing and execution
//!
//! This module provides functionality to:
//! - Load action configurations from JSON files
//! - Execute actions by their numeric IDs
//!
//! # Example JSON Configuration
//!
//! ```json
//! {
//!   "default_backend": "win32",
//!   "actions": [
//!     {"index": 0, "name": "jump", "type": "key", "key": "space"},
//!     {"index": 1, "name": "copy", "type": "key", "key": "ctrl+c"},
//!     {"index": 2, "name": "move", "type": "mouse_move", "x": 100, "y": 200},
//!     {"index": 3, "name": "hello", "type": "text", "content": "Hello World"}
//!   ]
//! }
//! ```

pub mod config;
pub mod executor;
pub mod history;
pub mod state;
pub mod types;
pub mod validation;
