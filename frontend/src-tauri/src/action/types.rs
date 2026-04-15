//! Action type definitions for OTAcle
//!
//! Defines the supported action types that can be configured via JSON.

use serde::{Deserialize, Serialize};

/// Variable definition - maps a parameter name to an action field name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    /// The parameter name used in ZMQ params
    pub param_name: String,
    /// The field name in this action to override
    pub field_name: String,
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
        InputBackend::Win32
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
    /// Number of clicks
    #[serde(default = "default_click_count")]
    pub count: u32,
    /// Interval between clicks in milliseconds (default: 0)
    #[serde(default)]
    pub interval_ms: Option<u64>,
    /// Hold time in milliseconds (default: 5ms, prevents events being dropped)
    #[serde(default = "default_mouse_hold_time")]
    pub hold_time_ms: u64,
    /// Override the default input backend for this action
    #[serde(default)]
    pub backend: Option<InputBackend>,
    /// Dynamic variables for runtime parameter substitution
    #[serde(default)]
    pub variables: Vec<Variable>,
}

fn default_click_count() -> u32 {
    1
}

fn default_mouse_hold_time() -> u64 {
    5
}

/// Mouse move action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
    /// Dynamic variables for runtime parameter substitution
    #[serde(default)]
    pub variables: Vec<Variable>,
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

/// A single action item with optional name and the action data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActionItem {
    /// Optional action name (uses array index as name if not provided)
    #[serde(default)]
    pub name: Option<String>,
    /// The action data
    #[serde(flatten)]
    pub data: Action,
}


/// Action configuration as a list of action items
pub type ActionConfigList = Vec<ActionItem>;

/// Action configuration mapping action index (u32) to Action
/// @deprecated Use ActionConfigList instead
pub type ActionConfig = std::collections::HashMap<u32, Action>;
