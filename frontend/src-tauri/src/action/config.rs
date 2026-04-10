//! JSON configuration parsing and validation for action config

use super::types::{Action, ActionConfig};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

lazy_static! {
    /// Global action configuration storage
    static ref ACTION_CONFIG: Mutex<Option<ActionConfig>> = Mutex::new(None);
}

/// Load action configuration from a JSON file
///
/// The JSON format maps action IDs (as strings) to action definitions.
/// Example:
/// ```json
/// {
///   "0": {"type": "key", "key": "space"},
///   "1": {"type": "mouse_click", "button": "left", "count": 2}
/// }
/// ```
pub fn load_config(path: &str) -> Result<ActionConfig, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    // Parse JSON into a HashMap with String keys first
    let string_config: HashMap<String, Action> =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Convert String keys to u32
    let mut result: ActionConfig = HashMap::new();
    for (key, action) in string_config {
        let id = key
            .parse::<u32>()
            .map_err(|_| format!("Invalid action ID '{}': must be a non-negative integer", key))?;
        result.insert(id, action);
    }

    // Validate the loaded config
    validate_config(&result)?;

    Ok(result)
}

/// Load configuration into global storage
pub fn load_config_with_backend(path: &str, _backend: super::types::InputBackend) -> Result<(), String> {
    // Backend is now specified per-call, not per-config
    let config = load_config(path)?;

    let mut global = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;

    *global = Some(config);

    Ok(())
}

/// Get currently loaded configuration
pub fn get_config() -> Result<ActionConfig, String> {
    let global = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;

    global
        .clone()
        .ok_or_else(|| "No configuration loaded. Call load_action_config first.".to_string())
}

/// Validate a single action
fn validate_action(action: &Action) -> Result<(), String> {
    match action {
        Action::Key(key_action) => {
            if key_action.key.is_empty() {
                return Err("Key name cannot be empty".to_string());
            }
        }
        Action::KeySequence(seq_action) => {
            if seq_action.keys.is_empty() {
                return Err("Key sequence cannot be empty".to_string());
            }
            for key in &seq_action.keys {
                if key.is_empty() {
                    return Err("Key in sequence cannot be empty".to_string());
                }
            }
        }
        Action::MouseClick(click_action) => {
            if click_action.count == 0 {
                return Err("Click count must be at least 1".to_string());
            }
        }
        Action::MouseMove(_) => {
            // x, y can be any value - no validation needed
        }
        Action::Text(text_action) => {
            if text_action.content.is_empty() {
                return Err("Text content cannot be empty".to_string());
            }
        }
    }
    Ok(())
}

/// Validate entire configuration
pub fn validate_config(config: &ActionConfig) -> Result<(), String> {
    if config.is_empty() {
        return Err("Configuration is empty".to_string());
    }

    for (id, action) in config {
        validate_action(action).map_err(|e| format!("Action {}: {}", id, e))?;
    }

    Ok(())
}
