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
//!   "0": {"type": "key", "key": "space"},
//!   "1": {"type": "key_sequence", "keys": ["ctrl", "c"], "interval_ms": 10},
//!   "2": {"type": "mouse_click", "button": "left", "count": 2},
//!   "3": {"type": "mouse_move", "x": 100, "y": 200},
//!   "4": {"type": "text", "content": "Hello World"}
//! }
//! ```

pub mod config;
pub mod executor;
pub mod resolver;
pub mod types;
