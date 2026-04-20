//! 通信模块的配置文件管理

use std::fs;

use crate::communication::state;
use crate::communication::types::CommConfig;

/// 从 JSON 文件加载通信配置
pub fn load_config(path: &str) -> Result<CommConfig, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: CommConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    // 保存到全局状态
    state::set_config(config.clone())?;

    Ok(config)
}

/// 保存通信配置到 JSON 文件
pub fn save_config(path: &str, config: &CommConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    // 更新全局状态
    state::set_config(config.clone())?;

    Ok(())
}

/// 获取当前配置
pub fn get_config() -> Result<CommConfig, String> {
    state::get_config()
}

/// 设置 PULL 地址
pub fn set_pull_address(addr: String) -> Result<(), String> {
    let mut config = state::get_config()?;
    config.pull_address = addr;
    state::set_config(config)
}

/// 获取 PULL 地址
pub fn get_pull_address() -> Result<String, String> {
    let config = state::get_config()?;
    Ok(config.pull_address)
}

/// 设置 PUB 地址
pub fn set_pub_address(addr: String) -> Result<(), String> {
    let mut config = state::get_config()?;
    config.pub_address = addr;
    state::set_config(config)
}

/// 获取 PUB 地址
pub fn get_pub_address() -> Result<String, String> {
    let config = state::get_config()?;
    Ok(config.pub_address)
}
