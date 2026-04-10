//! Action type definitions for OTAcle
//!
//! Defines the supported action types that can be configured via JSON.

use serde::{Deserialize, Serialize};

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

/// Key action - single key operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAction {
    /// Key name (e.g., "space", "enter", "a", "ctrl")
    pub key: String,
    /// Optional hold time in milliseconds (press and hold)
    #[serde(default)]
    pub hold_time_ms: Option<u64>,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// Key sequence action - multiple keys pressed in sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeySequenceAction {
    /// List of keys to press in order
    pub keys: Vec<String>,
    /// Interval between keys in milliseconds
    #[serde(default)]
    pub interval_ms: Option<u64>,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// Mouse click action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MouseClickAction {
    /// Mouse button to click
    pub button: MouseButton,
    /// Number of clicks (default: 1)
    #[serde(default = "default_click_count")]
    pub count: u32,
    /// Interval between clicks in milliseconds (default: 0)
    #[serde(default)]
    pub interval_ms: Option<u64>,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

fn default_click_count() -> u32 {
    1
}

/// Mouse move action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MouseMoveAction {
    /// Target X coordinate
    pub x: i32,
    /// Target Y coordinate
    pub y: i32,
    /// Optional duration to move (in milliseconds)
    #[serde(default)]
    pub duration_ms: Option<u64>,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// Text input action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    /// Text input action
    Text(TextAction),
}

/// A single action item with index, optional name, and the action data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
