//! Observe 模块的配置加载和保存

use std::fs;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::observe::types::{ObserveConfig, CropRegion};

lazy_static! {
    /// 全局 observe 配置存储
    pub static ref OBSERVE_CONFIG: Mutex<Option<ObserveConfig>> = Mutex::new(None);
}

/// 获取当前全局配置
pub fn get_config() -> Result<ObserveConfig, String> {
    let global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;

    global.clone()
        .ok_or_else(|| "No configuration loaded".to_string())
}

/// 验证 ObserveConfig 的合法性
pub fn validate_config(config: &ObserveConfig) -> Result<(), String> {
    if config.capture.target_width == 0 {
        return Err("target_width must be non-zero".to_string());
    }
    if config.capture.target_height == 0 {
        return Err("target_height must be non-zero".to_string());
    }
    if config.capture.target_width > 7680 {
        return Err("target_width exceeds maximum (7680)".to_string());
    }
    if config.capture.target_height > 4320 {
        return Err("target_height exceeds maximum (4320)".to_string());
    }
    if config.capture.frame_rate == 0 {
        return Err("frame_rate must be non-zero".to_string());
    }
    if config.capture.frame_rate > 120 {
        return Err("frame_rate exceeds maximum (120)".to_string());
    }

    for (idx, region) in config.crop_regions.iter().enumerate() {
        if region.w == 0 || region.h == 0 {
            return Err(format!("Crop region {} has zero dimensions", idx));
        }
        if region.x + region.w > config.capture.target_width {
            return Err(format!("Crop region {} exceeds target width", idx));
        }
        if region.y + region.h > config.capture.target_height {
            return Err(format!("Crop region {} exceeds target height", idx));
        }
    }

    Ok(())
}

/// 更新全局配置（不保存到文件）
pub fn update_config(config: ObserveConfig) -> Result<(), String> {
    validate_config(&config)?;
    let mut global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;
    *global = Some(config);
    Ok(())
}

/// 添加裁剪区域到当前配置
pub fn add_crop_region(region: CropRegion) -> Result<(), String> {
    let mut global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;
    let config = global.take().ok_or("No configuration loaded")?;

    let mut new_config = config.clone();
    new_config.crop_regions.push(region);
    if let Err(e) = validate_config(&new_config) {
        *global = Some(config);
        return Err(e);
    }
    *global = Some(new_config);
    Ok(())
}

/// 从当前配置删除裁剪区域
pub fn remove_crop_region(index: usize) -> Result<(), String> {
    let mut global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;
    let config = global.take().ok_or("No configuration loaded")?;

    if index >= config.crop_regions.len() {
        *global = Some(config);
        return Err(format!("Crop region index {} out of range", index));
    }

    let mut new_config = config.clone();
    new_config.crop_regions.remove(index);
    if let Err(e) = validate_config(&new_config) {
        *global = Some(config);
        return Err(e);
    }
    *global = Some(new_config);
    Ok(())
}

/// 从 JSON 文件加载 observe 配置
pub fn load_config(path: &str) -> Result<ObserveConfig, String> {
    if !std::path::Path::new(path).exists() {
        // 如果文件不存在，返回默认配置
        let default_config = ObserveConfig::default();
        return Ok(default_config);
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: ObserveConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    // 验证配置合法性
    validate_config(&config)?;

    // 保存到全局状态
    let mut global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;
    *global = Some(config.clone());

    Ok(config)
}

/// 将 observe 配置保存到 JSON 文件
pub fn save_config(path: &str, config: &ObserveConfig) -> Result<(), String> {
    validate_config(config)?;

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    let mut global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;
    *global = Some(config.clone());

    Ok(())
}

