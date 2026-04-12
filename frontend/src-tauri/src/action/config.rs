//! JSON configuration parsing and validation for action config

use super::types::{Action, ActionConfig, ActionConfigList, ActionItem, InputBackend, PlaceholderString};
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
    /// Global placeholder name to action index mapping (for uniqueness validation)
    static ref PLACEHOLDER_MAP: Mutex<Option<HashMap<String, u32>>> = Mutex::new(None);
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
    let (config, default_backend, _placeholder_map) = load_config_with_validation(path)?;

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
            for item in &seq_action.keys {
                if item.key.is_empty() {
                    return Err("Key in sequence cannot be empty".to_string());
                }
            }
        }
        Action::MouseClick(click_action) => {
            if let Ok(count) = click_action.count.parse::<u32>() {
                if count == 0 {
                    return Err("Click count must be at least 1".to_string());
                }
            }
            // If parsing fails, let it fail at execution time
        }
        Action::MouseMove(_) => {
            // x, y can be any value - no validation needed
        }
        Action::MouseScroll(scroll_action) => {
            if scroll_action.amount == 0 {
                return Err("Scroll amount must not be zero".to_string());
            }
        }
        Action::Delay(delay_action) => {
            // duration can be any value - allow 0 for no-op
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

/// Collect all placeholder names from an action
fn collect_placeholders_from_action(action: &Action) -> Vec<String> {
    match action {
        Action::Key(key_action) => {
            let ph = PlaceholderString::new(&key_action.key);
            if ph.has_placeholder() {
                ph.extract_placeholders()
            } else {
                Vec::new()
            }
        }
        Action::KeySequence(_) => {
            // Key sequences don't support placeholders in keys currently
            Vec::new()
        }
        Action::MouseClick(click_action) => {
            let ph = PlaceholderString::new(&click_action.count);
            if ph.has_placeholder() {
                ph.extract_placeholders()
            } else {
                Vec::new()
            }
        }
        Action::MouseMove(move_action) => {
            let mut names = Vec::new();
            let x_ph = PlaceholderString::new(&move_action.x);
            if x_ph.has_placeholder() {
                names.extend(x_ph.extract_placeholders());
            }
            let y_ph = PlaceholderString::new(&move_action.y);
            if y_ph.has_placeholder() {
                names.extend(y_ph.extract_placeholders());
            }
            names
        }
        Action::MouseScroll(_) => Vec::new(),
        Action::Delay(_) => Vec::new(),
        Action::Text(text_action) => {
            let ph = PlaceholderString::new(&text_action.content);
            if ph.has_placeholder() {
                ph.extract_placeholders()
            } else {
                Vec::new()
            }
        }
    }
}

/// Validate that all placeholder names are unique across actions
/// Returns the placeholder map (name -> action index) if valid
fn validate_placeholder_uniqueness(config: &ActionConfig) -> Result<HashMap<String, u32>, String> {
    let mut placeholder_map: HashMap<String, u32> = HashMap::new();
    let mut duplicates: Vec<(String, u32, u32)> = Vec::new();

    for (&action_idx, action) in config {
        for placeholder_name in collect_placeholders_from_action(action) {
            if let Some(&existing_idx) = placeholder_map.get(&placeholder_name) {
                if existing_idx != action_idx {
                    duplicates.push((placeholder_name, existing_idx, action_idx));
                }
            } else {
                placeholder_map.insert(placeholder_name, action_idx);
            }
        }
    }

    if !duplicates.is_empty() {
        let msg = duplicates
            .iter()
            .map(|(name, idx1, idx2)| format!("'{}' used in action {} and {}", name, idx1, idx2))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(format!("Duplicate placeholder names found: {}", msg));
    }

    Ok(placeholder_map)
}

/// Load and validate configuration with placeholder uniqueness check
pub fn load_config_with_validation(path: &str) -> Result<(ActionConfig, InputBackend, HashMap<String, u32>), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

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

    // Validate placeholder uniqueness and get the placeholder map
    let placeholder_map = validate_placeholder_uniqueness(&result)?;

    // Store the list for later access
    let mut global_list = ACTION_LIST.lock()
        .map_err(|_| "Failed to lock action list".to_string())?;
    *global_list = Some(action_list);

    // Store the placeholder map
    let mut global_placeholder_map = PLACEHOLDER_MAP.lock()
        .map_err(|_| "Failed to lock placeholder map".to_string())?;
    *global_placeholder_map = Some(placeholder_map.clone());

    Ok((result, config.default_backend, placeholder_map))
}

/// Get the placeholder name to action index mapping
pub fn get_placeholder_map() -> Result<HashMap<String, u32>, String> {
    let global = PLACEHOLDER_MAP
        .lock()
        .map_err(|_| "Failed to lock placeholder map".to_string())?;

    global
        .clone()
        .ok_or_else(|| "No configuration loaded. Call load_action_config first.".to_string())
}

/// Get the next available action index (max index + 1, or 0 if empty)
pub fn get_next_available_index() -> Result<u32, String> {
    let list = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    match list.as_ref() {
        Some(items) if !items.is_empty() => {
            let max_index = items.iter().map(|a| a.index).max().unwrap();
            Ok(max_index + 1)
        }
        _ => Ok(0),
    }
}

/// Create a new action in memory and return its assigned index
pub fn create_action(action: Action, name: Option<String>) -> Result<u32, String> {
    // Validate the action first
    validate_action(&action)?;

    let mut list = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    let index = match list.as_ref() {
        Some(items) if !items.is_empty() => {
            items.iter().map(|a| a.index).max().unwrap() + 1
        }
        _ => 0,
    };

    let item = ActionItem {
        index,
        name,
        data: action.clone(),
    };

    // Ensure list is initialized
    if list.is_none() {
        *list = Some(Vec::new());
    }

    if let Some(ref mut items) = *list {
        items.push(item);
    }

    // Also update the HashMap
    let mut config = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;
    if let Some(ref mut map) = *config {
        map.insert(index, action);
    } else {
        let mut map = HashMap::new();
        map.insert(index, action);
        *config = Some(map);
    }

    Ok(index)
}

/// Update an existing action by index
pub fn update_action(index: u32, action: Action, name: Option<String>) -> Result<(), String> {
    // Validate the action first
    validate_action(&action)?;

    // Update in ACTION_LIST
    {
        let mut list = ACTION_LIST
            .lock()
            .map_err(|_| "Failed to lock action list".to_string())?;

        let found = if let Some(ref mut items) = *list {
            if let Some(item) = items.iter_mut().find(|a| a.index == index) {
                item.data = action.clone();
                item.name = name;
                true
            } else {
                false
            }
        } else {
            false
        };

        if !found {
            return Err(format!("Action {} not found", index));
        }
    }

    // Update in ACTION_CONFIG HashMap
    {
        let mut config = ACTION_CONFIG
            .lock()
            .map_err(|_| "Failed to lock configuration".to_string())?;

        if let Some(ref mut map) = *config {
            if let Some(existing) = map.get_mut(&index) {
                *existing = action;
            } else {
                return Err(format!("Action {} not found in config map", index));
            }
        } else {
            return Err("No configuration loaded".to_string());
        }
    }

    Ok(())
}

/// Delete an action by index
pub fn delete_action(index: u32) -> Result<(), String> {
    // Delete from ACTION_LIST
    {
        let mut list = ACTION_LIST
            .lock()
            .map_err(|_| "Failed to lock action list".to_string())?;

        let found = if let Some(ref mut items) = *list {
            let pos = items.iter().position(|a| a.index == index);
            if let Some(pos) = pos {
                items.remove(pos);
                true
            } else {
                false
            }
        } else {
            false
        };

        if !found {
            return Err(format!("Action {} not found", index));
        }
    }

    // Delete from ACTION_CONFIG HashMap
    {
        let mut config = ACTION_CONFIG
            .lock()
            .map_err(|_| "Failed to lock configuration".to_string())?;

        if let Some(ref mut map) = *config {
            if map.remove(&index).is_none() {
                return Err(format!("Action {} not found in config map", index));
            }
        }
    }

    Ok(())
}

/// Save configuration to a JSON file
pub fn save_config(path: &str, default_backend: InputBackend, actions: &ActionConfigList) -> Result<(), String> {
    use serde::Serialize;

    #[derive(Serialize)]
    struct SaveConfig<'a> {
        default_backend: InputBackend,
        actions: &'a ActionConfigList,
    }

    let config = SaveConfig {
        default_backend: default_backend.clone(),
        actions,
    };

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    // Reload the saved config into global state
    load_config_with_backend(path, default_backend)?;

    Ok(())
}
