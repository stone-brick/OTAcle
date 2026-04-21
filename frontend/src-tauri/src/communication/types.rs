//! 通信模块类型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 通信配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommConfig {
    /// PULL 接收地址（Act 模块从 Python 接收命令）
    #[serde(default = "default_pull_address")]
    pub pull_address: String,
    /// PUB 发布地址（Observe 模块向 Python 发送图像）
    #[serde(default = "default_pub_address")]
    pub pub_address: String,
    /// 默认输入后端
    #[serde(default = "default_backend")]
    pub default_backend: String,
}

impl Default for CommConfig {
    fn default() -> Self {
        Self {
            pull_address: default_pull_address(),
            pub_address: default_pub_address(),
            default_backend: default_backend(),
        }
    }
}

fn default_pull_address() -> String {
    "tcp://127.0.0.1:5555".to_string()
}

fn default_pub_address() -> String {
    "tcp://127.0.0.1:5556".to_string()
}

fn default_backend() -> String {
    "win32".to_string()
}

/// 命令消息格式（Act 模块使用）
#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    /// 执行列表，按顺序对应配置文件中 action 的 index
    /// 例如 [true, false, true] 表示执行 index=0 和 index=2 的动作
    pub execute: Vec<bool>,
    /// 动态参数映射 (param_name -> value)
    /// 如果省略或为空，使用配置文件中的默认值
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

/// 连接状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connected,
    Error,
}

/// PUB 发布者状态（每个会话一个）
#[derive(Debug)]
pub struct PubState {
    pub connection_state: std::sync::Arc<std::sync::Mutex<ConnectionState>>,
    pub messages_sent: std::sync::atomic::AtomicU64,
    pub bytes_sent: std::sync::atomic::AtomicU64,
    pub last_error: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

impl Default for PubState {
    fn default() -> Self {
        Self::new()
    }
}

impl PubState {
    pub fn new() -> Self {
        Self {
            connection_state: std::sync::Arc::new(std::sync::Mutex::new(
                ConnectionState::Disconnected,
            )),
            messages_sent: std::sync::atomic::AtomicU64::new(0),
            bytes_sent: std::sync::atomic::AtomicU64::new(0),
            last_error: std::sync::Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn set_connected(&self) {
        if let Ok(mut state) = self.connection_state.lock() {
            *state = ConnectionState::Connected;
        }
        if let Ok(mut err) = self.last_error.lock() {
            *err = None;
        }
    }

    pub fn set_disconnected(&self) {
        if let Ok(mut state) = self.connection_state.lock() {
            *state = ConnectionState::Disconnected;
        }
    }

    pub fn set_error(&self, error: String) {
        if let Ok(mut state) = self.connection_state.lock() {
            *state = ConnectionState::Error;
        }
        if let Ok(mut err) = self.last_error.lock() {
            *err = Some(error);
        }
    }

    pub fn get_connection_state(&self) -> ConnectionState {
        self.connection_state
            .lock()
            .ok()
            .map(|s| *s)
            .unwrap_or(ConnectionState::Disconnected)
    }

    pub fn messages_sent(&self) -> u64 {
        self.messages_sent.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn bytes_sent(&self) -> u64 {
        self.bytes_sent.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn last_error(&self) -> Option<String> {
        self.last_error.lock().ok().and_then(|g| g.clone())
    }

    pub fn add_messages_sent(&self, count: u64) {
        self.messages_sent
            .fetch_add(count, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn add_bytes_sent(&self, bytes: u64) {
        self.bytes_sent
            .fetch_add(bytes, std::sync::atomic::Ordering::SeqCst);
    }
}

/// 图像帧消息（Observe 模块使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMessage {
    pub width: u32,
    pub height: u32,
    pub timestamp: u64,
    pub frame_id: u64,
    pub data: Vec<CropBlock>,
}

/// 单个裁切块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropBlock {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub image: String,
}
