//! Action type definitions for OTAcle
//!
//! Defines the supported action types that can be configured via JSON.

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Regex pattern for matching placeholders like {{variable_name}}
static PLACEHOLDER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\{\{([a-zA-Z_][a-zA-Z0-9_]*)\}\}").unwrap()
});

/// Support parameter value types for placeholder substitution
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ParamValue {
    /// Integer value (i32)
    Integer(i32),
    /// Unsigned integer value (u32)
    Unsigned(u32),
    /// String value
    String(String),
}

impl ParamValue {
    /// Convert to i32 for numeric fields
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            ParamValue::Integer(i) => Some(*i),
            ParamValue::Unsigned(u) => Some(*u as i32),
            ParamValue::String(s) => s.parse().ok(),
        }
    }

    /// Convert to u32 for numeric fields
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            ParamValue::Integer(i) => Some(*i as u32),
            ParamValue::Unsigned(u) => Some(*u),
            ParamValue::String(s) => s.parse().ok(),
        }
    }

    /// Convert to String for text fields
    pub fn as_string(&self) -> String {
        match self {
            ParamValue::Integer(i) => i.to_string(),
            ParamValue::Unsigned(u) => u.to_string(),
            ParamValue::String(s) => s.clone(),
        }
    }
}

/// A string that may contain placeholders like {{variable_name}}
#[derive(Debug, Clone, PartialEq)]
pub struct PlaceholderString(String);

impl PlaceholderString {
    /// Create a new PlaceholderString
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Check if the string contains any placeholders
    pub fn has_placeholder(&self) -> bool {
        PLACEHOLDER_REGEX.is_match(&self.0)
    }

    /// Resolve placeholders using the provided parameters
    /// Returns error if any placeholder is missing from params
    pub fn resolve(&self, params: &HashMap<String, ParamValue>) -> Result<String, String> {
        // Check all placeholders exist in params
        let mut missing = Vec::new();
        for caps in PLACEHOLDER_REGEX.captures_iter(&self.0) {
            let var_name = &caps[1];
            if !params.contains_key(var_name) {
                missing.push(var_name.to_string());
            }
        }

        if !missing.is_empty() {
            return Err(format!("Missing parameters: {}", missing.join(", ")));
        }

        // Replace all placeholders
        let result = PLACEHOLDER_REGEX.replace_all(&self.0, |caps: &regex::Captures| {
            let var_name = &caps[1];
            params.get(var_name).map(|v| v.as_string()).unwrap()
        });

        Ok(result.into_owned())
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parse as i32 (for numeric fields like x, y coordinates)
    pub fn parse_i32(&self) -> Option<i32> {
        self.0.parse().ok()
    }

    /// Parse as u32 (for count fields)
    pub fn parse_u32(&self) -> Option<u32> {
        self.0.parse().ok()
    }

    /// Extract all placeholder names from the string
    pub fn extract_placeholders(&self) -> Vec<String> {
        PLACEHOLDER_REGEX
            .captures_iter(&self.0)
            .map(|caps| caps[1].to_string())
            .collect()
    }
}

impl From<String> for PlaceholderString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for PlaceholderString {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Input backend type - determines how input is sent to target window
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputBackend {
    /// Use enigo library - cross-platform, relies on foreground window
    Enigo,
    /// Use Windows API - sends directly to target window, no foreground required
    Win32,
}

impl Default for InputBackend {
    fn default() -> Self {
        InputBackend::Enigo
    }
}

/// Mouse button variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Mouse scroll direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Key action - single key operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyAction {
    /// Key name (e.g., "space", "enter", "a", "ctrl")
    pub key: String,
    /// Hold time in milliseconds (default: 5ms, prevents events being dropped)
    #[serde(default = "default_key_hold_time")]
    pub hold_time_ms: u64,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// Key sequence action - multiple keys pressed in sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeySequenceAction {
    /// List of keys to press in order
    pub keys: Vec<KeySequenceItem>,
    /// Default interval between keys in milliseconds (default: 5ms)
    #[serde(default = "default_interval_ms")]
    pub default_interval_ms: u64,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// A single key item in a key sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeySequenceItem {
    /// Key name (e.g., "space", "enter", "ctrl")
    pub key: String,
    /// Hold time in milliseconds (default: 5ms, prevents events being dropped)
    #[serde(default = "default_key_hold_time")]
    pub hold_time_ms: u64,
    /// Interval to the next key in milliseconds (overrides default_interval_ms)
    #[serde(default)]
    pub interval_ms: Option<u64>,
}

fn default_interval_ms() -> u64 {
    5
}

fn default_key_hold_time() -> u64 {
    5
}

/// Mouse click action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MouseClickAction {
    /// Mouse button to click
    pub button: MouseButton,
    /// Number of clicks (supports placeholder like "{{click_count}}")
    #[serde(default = "default_click_count")]
    pub count: String,
    /// Interval between clicks in milliseconds (default: 0)
    #[serde(default)]
    pub interval_ms: Option<u64>,
    /// Hold time in milliseconds (default: 5ms, prevents events being dropped)
    #[serde(default = "default_mouse_hold_time")]
    pub hold_time_ms: u64,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

fn default_click_count() -> String {
    "1".to_string()
}

fn default_mouse_hold_time() -> u64 {
    5
}

/// Mouse move action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MouseMoveAction {
    /// Target X coordinate (supports placeholder like "{{target_x}}")
    pub x: String,
    /// Target Y coordinate (supports placeholder like "{{target_y}}")
    pub y: String,
    /// Optional duration to move (in milliseconds)
    #[serde(default)]
    pub duration_ms: Option<u64>,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// Mouse scroll action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MouseScrollAction {
    /// Scroll direction
    pub direction: ScrollDirection,
    /// Scroll amount in "clicks" (default: 1, Windows default wheel delta is 120)
    #[serde(default = "default_scroll_amount")]
    pub amount: u32,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

fn default_scroll_amount() -> u32 {
    1
}

/// Delay/wait action - pauses execution for specified duration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DelayAction {
    /// Duration to wait in milliseconds
    pub duration_ms: u64,
}

/// Text input action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextAction {
    /// Text content to type
    pub content: String,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// Action type enum - discriminated union for all action types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// Single key operation
    Key(KeyAction),
    /// Multiple keys pressed in sequence
    KeySequence(KeySequenceAction),
    /// Mouse click action
    MouseClick(MouseClickAction),
    /// Mouse move action
    MouseMove(MouseMoveAction),
    /// Mouse scroll action
    MouseScroll(MouseScrollAction),
    /// Delay/wait action
    Delay(DelayAction),
    /// Text input action
    Text(TextAction),
}

/// A single action item with index, optional name, and the action data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActionItem {
    /// Action index (used as identifier and fallback name)
    pub index: u32,
    /// Optional action name (uses index as name if not provided)
    #[serde(default)]
    pub name: Option<String>,
    /// The action data
    #[serde(flatten)]
    pub data: Action,
}

impl ActionItem {
    /// Get the effective name of this action
    pub fn effective_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| self.index.to_string())
    }
}

/// Action configuration as a list of action items
pub type ActionConfigList = Vec<ActionItem>;

/// Action configuration mapping action index (u32) to Action
/// @deprecated Use ActionConfigList instead
pub type ActionConfig = std::collections::HashMap<u32, Action>;
