//! Observe 模块的类型定义

use serde::{Deserialize, Serialize};

// ============================================================================
// 配置类型
// ============================================================================

/// Observe module configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserveConfig {
    pub capture: CaptureConfig,
    pub zmq: ZmqConfig,
    #[serde(default)]
    pub crop_regions: Vec<CropRegion>,
}

impl Default for ObserveConfig {
    fn default() -> Self {
        Self {
            capture: CaptureConfig::default(),
            zmq: ZmqConfig::default(),
            crop_regions: Vec::new(),
        }
    }
}

/// Capture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConfig {
    #[serde(default = "default_frame_rate")]
    pub frame_rate: u32,
    #[serde(default = "default_target_width")]
    pub target_width: u32,
    #[serde(default = "default_target_height")]
    pub target_height: u32,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            frame_rate: default_frame_rate(),
            target_width: default_target_width(),
            target_height: default_target_height(),
        }
    }
}

/// ZMQ configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZmqConfig {
    #[serde(default = "default_zmq_address")]
    pub address: String,
}

impl Default for ZmqConfig {
    fn default() -> Self {
        Self {
            address: default_zmq_address(),
        }
    }
}

/// Crop region in the captured frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropRegion {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

// ============================================================================
// 帧消息类型
// ============================================================================

/// Frame message sent via ZMQ PUB and Tauri events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMessage {
    pub width: u32,
    pub height: u32,
    pub timestamp: u64,
    pub frame_id: u64,
    pub data: Vec<CropBlock>,
}

/// Single crop block within a frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropBlock {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    /// Base64 encoded RGBA8 image data
    pub image: String,
}

// ============================================================================
// 默认值函数
// ============================================================================

fn default_frame_rate() -> u32 {
    20
}

fn default_target_width() -> u32 {
    640
}

fn default_target_height() -> u32 {
    480
}

fn default_zmq_address() -> String {
    "tcp://127.0.0.1:5556".to_string()
}
