//! Observe 模块的类型定义

use serde::{Deserialize, Serialize};

// ============================================================================
// 配置类型
// ============================================================================

/// Observe module configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ObserveConfig {
    pub capture: CaptureConfig,
    #[serde(default)]
    pub crop_regions: Vec<CropRegion>,
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

/// Crop region in the captured frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropRegion {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
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
