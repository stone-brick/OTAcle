//! JSON configuration parsing and validation for action config

use super::types::{Action, ActionConfig, ActionConfigList, InputBackend};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

lazy_static! {
    /// Global action configuration storage (maps index to Action)
    static ref ACTION_CONFIG: Mutex<Option<ActionConfig>> = Mutex::new(None);
    /// Global action list storage (preserves order and names)
    static ref ACTION_LIST: Mutex<Option<ActionConfigList>> = Mutex::new(None);
    /// Global default backend for actions without explicit backend
    static ref DEFAULT_BACKEND: Mutex<InputBackend> = Mutex::new(InputBackend::Win32);
}

/// JSON config structure
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Default input backend when action doesn't specify one
    #[serde(default = "default_win32")]
    pub default_backend: InputBackend,
    /// List of action items
    pub actions: Vec<ActionItemWrapper>,
}

/// Wrapper for parsing action items from JSON
#[derive(Debug, Clone, Deserialize)]
pub struct ActionItemWrapper {
    /// Action index (used as identifier and fallback name)
    pub index: u32,
    /// Optional action name (uses index as name if not provided)
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten)]
    pub action: Action,
}

fn default_win32() -> InputBackend {
    InputBackend::Win32
}

/// Load action configuration from a JSON file
///
/// **JSON format**:
/// ```json
/// {
///   "default_backend": "win32",
///   "actions": [
///     {"index": 0, "name": "jump", "type": "key", "key": "space"},
///     {"index": 1, "type": "key", "key": "ctrl+c", "backend": "enigo"}
///   ]
/// }
/// ```
pub fn load_config(path: &str) -> Result<(ActionConfig, InputBackend), String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: Config = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    let mut result: ActionConfig = HashMap::new();
    let mut action_list: ActionConfigList = Vec::new();

    for item in config.actions {
        result.insert(item.index, item.action.clone());
        action_list.push(super::types::ActionItem {
            index: item.index,
            name: item.name,
            data: item.action,
        });
    }

    validate_config(&result)?;

    // Store the list for later access
    let mut global_list = ACTION_LIST.lock()
        .map_err(|_| "Failed to lock action list".to_string())?;
    *global_list = Some(action_list);

    Ok((result, config.default_backend))
}

/// Load configuration into global storage
pub fn load_config_with_backend(path: &str, _backend: InputBackend) -> Result<(), String> {
    // Backend is now specified per-config file, not per-call
    let (config, default_backend) = load_config(path)?;

    let mut global_config = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;

    let mut global_backend = DEFAULT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock default backend".to_string())?;

    *global_config = Some(config);
    *global_backend = default_backend;

    Ok(())
}

/// Get currently loaded configuration (maps index to Action)
pub fn get_config() -> Result<ActionConfig, String> {
    let global = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;

    global
        .clone()
        .ok_or_else(|| "No configuration loaded. Call load_action_config first.".to_string())
}

/// Get the action list with names (preserves order and names)
pub fn get_action_list() -> Result<ActionConfigList, String> {
    let global = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    global
        .clone()
        .ok_or_else(|| "No configuration loaded. Call load_action_config first.".to_string())
}

/// Get the default backend from loaded configuration
pub fn get_default_backend() -> Result<InputBackend, String> {
    let global = DEFAULT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock default backend".to_string())?;

    Ok(global.clone())
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
