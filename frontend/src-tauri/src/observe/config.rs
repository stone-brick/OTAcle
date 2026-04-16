//! Observe 模块的配置加载和保存

use std::fs;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::observe::types::ObserveConfig;

lazy_static! {
    /// 全局 observe 配置存储
    pub static ref OBSERVE_CONFIG: Mutex<Option<ObserveConfig>> = Mutex::new(None);
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

    // 保存到全局状态
    let mut global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;
    *global = Some(config.clone());

    Ok(config)
}

/// 将 observe 配置保存到 JSON 文件
pub fn save_config(path: &str, config: &ObserveConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    // 更新全局状态
    let mut global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;
    *global = Some(config.clone());

    Ok(())
}

