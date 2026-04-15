//! JSON configuration parsing and validation for action config

use crate::input;
use super::types::{Action, ActionConfig, ActionConfigList, ActionItem, InputBackend};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

lazy_static! {
    /// Global action configuration storage (maps index to Action)
    static ref ACTION_CONFIG: Mutex<Option<ActionConfig>> = Mutex::new(None);
    /// Global action list storage (preserves order and names)
    static ref ACTION_LIST: Mutex<Option<ActionConfigList>> = Mutex::new(None);
    /// Global default backend for actions without explicit backend
    static ref DEFAULT_BACKEND: Mutex<InputBackend> = Mutex::new(InputBackend::Win32);
    /// Global target window for action execution (HWND)
    pub static ref TARGET_WINDOW: Mutex<Option<isize>> = Mutex::new(None);
    /// Global execution backend override
    pub static ref EXECUTION_BACKEND: Mutex<InputBackend> = Mutex::new(InputBackend::Win32);
}

/// Maximum history entries
const MAX_HISTORY_SIZE: usize = 50;

/// History entry for undo/redo
#[derive(Clone)]
pub struct HistoryEntry {
    pub actions: ActionConfigList,
    pub default_backend: InputBackend,
}

lazy_static! {
    /// Undo history stack
    pub static ref UNDO_STACK: Mutex<Vec<HistoryEntry>> = Mutex::new(Vec::new());
    /// Redo history stack
    pub static ref REDO_STACK: Mutex<Vec<HistoryEntry>> = Mutex::new(Vec::new());
    /// Original state for discard (loaded or last saved)
    pub static ref ORIGINAL_ENTRY: Mutex<Option<HistoryEntry>> = Mutex::new(None);
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
    /// Optional action name
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

    for (idx, item) in config.actions.into_iter().enumerate() {
        let index = idx as u32;
        result.insert(index, item.action.clone());
        action_list.push(super::types::ActionItem {
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
    *global_backend = default_backend.clone();

    // Set original entry for discard
    let action_list = ACTION_LIST.lock().map_err(|_| "Failed to lock action list")?;
    let mut original = ORIGINAL_ENTRY.lock().map_err(|_| "Failed to lock original")?;
    *original = Some(HistoryEntry {
        actions: action_list.clone().unwrap_or_default(),
        default_backend,
    });

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

/// Set the target window for action execution
pub fn set_target_window(window: Option<String>) -> Result<(), String> {
    let hwnd = match window {
        Some(spec) => {
            let search = input::parse_window_spec(&spec);

            // Empty spec means foreground window
            if search.title.is_none()
                && search.class_name.is_none()
                && search.hwnd.is_none()
                && search.pid.is_none()
                && search.exe_name.is_none()
            {
                let hwnd = unsafe { GetForegroundWindow() };
                if hwnd.0.is_null() {
                    return Err("No foreground window found".to_string());
                }
                hwnd.0 as isize
            } else {
                // Resolve window spec to HWND
                input::find_window(&search)
                    .ok_or_else(|| format!("Window not found: {}", spec))?
            }
        }
        None => {
            // Clear target window
            let mut global = TARGET_WINDOW
                .lock()
                .map_err(|_| "Failed to lock target window".to_string())?;
            *global = None;
            return Ok(());
        }
    };

    let mut global = TARGET_WINDOW
        .lock()
        .map_err(|_| "Failed to lock target window".to_string())?;
    *global = Some(hwnd);
    Ok(())
}

/// Get the current target window (HWND)
pub fn get_target_window() -> Option<isize> {
    TARGET_WINDOW
        .lock()
        .map(|g| g.clone())
        .unwrap_or(None)
}

/// Set the execution backend override
pub fn set_execution_backend(backend: InputBackend) -> Result<(), String> {
    let mut global = EXECUTION_BACKEND
        .lock()
        .map_err(|_| "Failed to lock execution backend".to_string())?;
    *global = backend;
    Ok(())
}

/// Get the current execution backend override
pub fn get_execution_backend() -> InputBackend {
    EXECUTION_BACKEND
        .lock()
        .map(|g| g.clone())
        .unwrap_or(InputBackend::Win32)
}

/// Set the default backend (with history tracking)
pub fn set_default_backend(backend: InputBackend) -> Result<(), String> {
    // Save current state to history before modification
    save_to_history()?;

    let mut global = DEFAULT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock default backend".to_string())?;
    *global = backend;
    Ok(())
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
            if click_action.count == 0 {
                return Err("Click count must be at least 1".to_string());
            }
        }
        Action::MouseMove(_) => {
            // x, y can be any value - no validation needed
        }
        Action::MouseScroll(scroll_action) => {
            if scroll_action.amount == 0 {
                return Err("Scroll amount must not be zero".to_string());
            }
        }
        Action::Delay(_delay_action) => {
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

// Save current state to undo history
pub fn save_to_history() -> Result<(), String> {
    let actions = get_action_list()?;
    let backend = get_default_backend()?;

    let entry = HistoryEntry {
        actions,
        default_backend: backend,
    };

    let mut undo = UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?;
    undo.push(entry);

    // Limit history size
    if undo.len() > MAX_HISTORY_SIZE {
        undo.remove(0);
    }

    // New action clears redo stack
    let mut redo = REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?;
    redo.clear();

    Ok(())
}

/// Get the next available action index (list length, or 0 if empty)
pub fn get_next_available_index() -> Result<u32, String> {
    let list = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    Ok(list.as_ref().map(|items| items.len()).unwrap_or(0) as u32)
}

/// Rebuild the HashMap from the current action list to ensure indices match positions
fn rebuild_config_from_list() -> Result<(), String> {
    let list = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    let new_config: ActionConfig = list
        .as_ref()
        .map(|items| {
            items
                .iter()
                .enumerate()
                .map(|(idx, item)| (idx as u32, item.data.clone()))
                .collect()
        })
        .unwrap_or_default();

    let mut config = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;
    *config = Some(new_config);

    Ok(())
}

/// Create a new action in memory and return its assigned index
pub fn create_action(action: Action, name: Option<String>) -> Result<u32, String> {
    // Save current state to history before modification
    save_to_history()?;

    // Validate the action first
    validate_action(&action)?;

    let mut list = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    // Index is the length of the list (append at end)
    let index = list.as_ref().map(|items| items.len()).unwrap_or(0) as u32;

    let item = ActionItem {
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
    // Save current state to history before modification
    save_to_history()?;

    // Validate the action first
    validate_action(&action)?;

    // Update in ACTION_LIST
    {
        let mut list = ACTION_LIST
            .lock()
            .map_err(|_| "Failed to lock action list".to_string())?;

        let found = if let Some(ref mut items) = *list {
            if let Some(item) = items.get_mut(index as usize) {
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
    // Save current state to history before modification
    save_to_history()?;

    // Delete from ACTION_LIST
    {
        let mut list = ACTION_LIST
            .lock()
            .map_err(|_| "Failed to lock action list".to_string())?;

        let found = if let Some(ref mut items) = *list {
            if (index as usize) < items.len() {
                items.remove(index as usize);
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

    // Rebuild HashMap from list to maintain index-to-position correspondence
    rebuild_config_from_list()?;

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

// Undo - restore previous state
pub fn undo() -> Result<(), String> {
    let mut undo = UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?;
    let mut redo = REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?;

    if undo.is_empty() {
        return Err("Nothing to undo".to_string());
    }

    // Save current state to redo stack
    let current = HistoryEntry {
        actions: get_action_list()?,
        default_backend: get_default_backend()?,
    };
    redo.push(current);

    // Restore previous state
    let prev = undo.pop().unwrap();
    reload_from_entry(&prev)?;

    Ok(())
}

// Redo - restore next state
pub fn redo() -> Result<(), String> {
    let mut undo = UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?;
    let mut redo = REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?;

    if redo.is_empty() {
        return Err("Nothing to redo".to_string());
    }

    // Save current state to undo stack
    let current = HistoryEntry {
        actions: get_action_list()?,
        default_backend: get_default_backend()?,
    };
    undo.push(current);

    // Restore next state
    let next = redo.pop().unwrap();
    reload_from_entry(&next)?;

    Ok(())
}

// Discard all changes - restore to original state (undoable)
pub fn discard_changes() -> Result<(), String> {
    // Save current state to history (so we can undo)
    save_to_history()?;

    // Restore from original
    let original = ORIGINAL_ENTRY.lock().map_err(|_| "Failed to lock original")?;
    let entry = original
        .clone()
        .ok_or_else(|| "No original state to discard to".to_string())?;
    drop(original);

    reload_from_entry(&entry)?;

    Ok(())
}

// Discard a specific action - restore to original state (undoable)
pub fn discard_action(index: u32) -> Result<(), String> {
    // Save current state to history (so we can undo)
    save_to_history()?;

    // Get original action at this index
    let original = ORIGINAL_ENTRY.lock().map_err(|_| "Failed to lock original")?;
    let entry = original
        .clone()
        .ok_or_else(|| "No original state to discard to".to_string())?;

    let original_action = entry
        .actions
        .get(index as usize)
        .ok_or_else(|| format!("No action at index {}", index))?;
    drop(original);

    // Update the specific action in global state
    let mut config = ACTION_CONFIG.lock().map_err(|_| "Failed to lock config")?;
    if let Some(ref mut c) = *config {
        c.insert(index, original_action.data.clone());
    }

    let mut list = ACTION_LIST.lock().map_err(|_| "Failed to lock action list")?;
    if let Some(ref mut l) = *list {
        l[index as usize] = original_action.clone();
    }

    Ok(())
}

// Reload state from a history entry
fn reload_from_entry(entry: &HistoryEntry) -> Result<(), String> {
    // Update global ACTION_CONFIG HashMap using array positions as indices
    let mut config = ACTION_CONFIG.lock().map_err(|_| "Failed to lock config")?;
    *config = Some(
        entry
            .actions
            .iter()
            .enumerate()
            .map(|(idx, a)| (idx as u32, a.data.clone()))
            .collect(),
    );

    // Update global ACTION_LIST
    let mut list = ACTION_LIST.lock().map_err(|_| "Failed to lock action list")?;
    *list = Some(entry.actions.clone());

    // Update global DEFAULT_BACKEND
    let mut backend = DEFAULT_BACKEND.lock().map_err(|_| "Failed to lock default backend")?;
    *backend = entry.default_backend.clone();

    Ok(())
}

// Clear undo/redo history
pub fn clear_history() -> Result<(), String> {
    UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?.clear();
    REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?.clear();
    Ok(())
}

// Get history status (undo count, redo count)
pub fn get_history_status() -> (usize, usize) {
    let undo_len = UNDO_STACK.lock().map(|g| g.len()).unwrap_or(0);
    let redo_len = REDO_STACK.lock().map(|g| g.len()).unwrap_or(0);
    (undo_len, redo_len)
}
