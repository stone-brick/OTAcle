//! OTAcle 的动作类型定义
//!
//! 定义可通过 JSON 配置的支持的动作类型。

use serde::{Deserialize, Serialize};

/// 变量定义 - 将参数名称映射到动作字段名称
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    /// ZMQ params 中使用的参数名称
    pub param_name: String,
    /// 此动作中要覆盖的字段名称
    pub field_name: String,
}

/// 输入后端类型 - 决定如何将输入发送到目标窗口
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputBackend {
    /// 使用 enigo 库 - 跨平台，依赖前台窗口
    Enigo,
    /// 使用 Windows API - 直接发送到目标窗口，无需前台
    Win32,
}

impl Default for InputBackend {
    fn default() -> Self {
        InputBackend::Win32
    }
}

/// 鼠标按钮变体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// 鼠标滚动方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

/// 按键动作 - 单键操作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyAction {
    /// 按键名称（例如 "space"、"enter"、"a"、"ctrl"）
    pub key: String,
    /// 按住时间（毫秒，默认 5ms，防止事件丢失）
    #[serde(default = "default_key_hold_time")]
    pub hold_time_ms: u64,
    /// 覆盖此动作的默认输入后端
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// 按键序列动作 - 按顺序按下多个按键
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeySequenceAction {
    /// 按顺序按下的按键列表
    pub keys: Vec<KeySequenceItem>,
    /// 按键之间的默认间隔（毫秒，默认 5ms）
    #[serde(default = "default_interval_ms")]
    pub default_interval_ms: u64,
    /// 覆盖此动作的默认输入后端
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// 按键序列中的单个按键项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeySequenceItem {
    /// 按键名称（例如 "space"、"enter"、"ctrl"）
    pub key: String,
    /// 按住时间（毫秒，默认 5ms，防止事件丢失）
    #[serde(default = "default_key_hold_time")]
    pub hold_time_ms: u64,
    /// 与下一个按键的间隔（毫秒，覆盖 default_interval_ms）
    #[serde(default)]
    pub interval_ms: Option<u64>,
}

fn default_interval_ms() -> u64 {
    5
}

fn default_key_hold_time() -> u64 {
    5
}

/// 鼠标点击动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MouseClickAction {
    /// 要点击的鼠标按钮
    pub button: MouseButton,
    /// 点击次数
    #[serde(default = "default_click_count")]
    pub count: u32,
    /// 点击之间的间隔（毫秒，默认 0）
    #[serde(default)]
    pub interval_ms: Option<u64>,
    /// 按住时间（毫秒，默认 5ms，防止事件丢失）
    #[serde(default = "default_mouse_hold_time")]
    pub hold_time_ms: u64,
    /// 覆盖此动作的默认输入后端
    #[serde(default)]
    pub backend: Option<InputBackend>,
    /// 用于运行时参数替换的动态变量
    #[serde(default)]
    pub variables: Vec<Variable>,
}

fn default_click_count() -> u32 {
    1
}

fn default_mouse_hold_time() -> u64 {
    5
}

/// 鼠标移动动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MouseMoveAction {
    /// 目标 X 坐标
    pub x: i32,
    /// 目标 Y 坐标
    pub y: i32,
    /// 可选的移动持续时间（毫秒）
    #[serde(default)]
    pub duration_ms: Option<u64>,
    /// 覆盖此动作的默认输入后端
    #[serde(default)]
    pub backend: Option<InputBackend>,
    /// 用于运行时参数替换的动态变量
    #[serde(default)]
    pub variables: Vec<Variable>,
}

/// 鼠标滚动动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MouseScrollAction {
    /// 滚动方向
    pub direction: ScrollDirection,
    /// 滚动量（"点击"数，默认 1，Windows 默认滚轮增量是 120）
    #[serde(default = "default_scroll_amount")]
    pub amount: u32,
    /// 覆盖此动作的默认输入后端
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

fn default_scroll_amount() -> u32 {
    1
}

/// 延迟/等待动作 - 按指定时长暂停执行
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DelayAction {
    /// 等待时长（毫秒）
    pub duration_ms: u64,
}

/// 文本输入动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextAction {
    /// 要输入的文本内容
    pub content: String,
    /// 覆盖此动作的默认输入后端
    #[serde(default)]
    pub backend: Option<InputBackend>,
}

/// 动作类型枚举 - 所有动作类型的可辨识联合
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// 单键操作
    Key(KeyAction),
    /// 按顺序按下多个按键
    KeySequence(KeySequenceAction),
    /// 鼠标点击动作
    MouseClick(MouseClickAction),
    /// 鼠标移动动作
    MouseMove(MouseMoveAction),
    /// 鼠标滚动动作
    MouseScroll(MouseScrollAction),
    /// 延迟/等待动作
    Delay(DelayAction),
    /// 文本输入动作
    Text(TextAction),
}

/// 带可选名称和动作数据的单个动作项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActionItem {
    /// 可选的动作名称（如果未提供则使用数组索引作为名称）
    #[serde(default)]
    pub name: Option<String>,
    /// 动作数据
    #[serde(flatten)]
    pub data: Action,
}


/// 动作配置作为动作项列表
pub type ActionConfigList = Vec<ActionItem>;

/// 动作配置，将动作索引（u32）映射到 Action
/// @deprecated 请改用 ActionConfigList
pub type ActionConfig = std::collections::HashMap<u32, Action>;
