//! 通信模块的配置文件管理

use std::fs;

use crate::communication::state;
use crate::communication::types::CommConfig;

/// 从 JSON 文件加载通信配置
pub fn load_config(path: &str) -> Result<CommConfig, String> {
    if !std::path::Path::new(path).exists() {
        // 如果文件不存在，返回默认配置
        let default_config = CommConfig::default();
        state::set_config(default_config.clone())?;
        return Ok(default_config);
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: CommConfig =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))?;

    // 验证配置合法性
    validate_config(&config)?;

    // 保存到全局状态
    state::set_config(config.clone())?;

    Ok(config)
}

/// 保存通信配置到 JSON 文件
pub fn save_config(path: &str, config: &CommConfig) -> Result<(), String> {
    validate_config(config)?;

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, json).map_err(|e| format!("Failed to write config file: {}", e))?;

    // 更新全局状态
    state::set_config(config.clone())?;

    Ok(())
}

/// 获取当前配置
pub fn get_config() -> Result<CommConfig, String> {
    state::get_config()
}

/// 验证 CommConfig 的合法性
pub fn validate_config(config: &CommConfig) -> Result<(), String> {
    // 验证 pull_address 格式
    if config.pull_address.is_empty() {
        return Err("pull_address cannot be empty".to_string());
    }
    if !config.pull_address.starts_with("tcp://")
        && !config.pull_address.starts_with("ipc://")
        && !config.pull_address.starts_with("inproc://")
    {
        return Err(format!(
            "pull_address must be a valid ZMQ address, got: {}",
            config.pull_address
        ));
    }

    // 验证 pub_address 格式
    if config.pub_address.is_empty() {
        return Err("pub_address cannot be empty".to_string());
    }
    if !config.pub_address.starts_with("tcp://")
        && !config.pub_address.starts_with("ipc://")
        && !config.pub_address.starts_with("inproc://")
    {
        return Err(format!(
            "pub_address must be a valid ZMQ address, got: {}",
            config.pub_address
        ));
    }

    // 验证 default_backend
    if config.default_backend != "win32" && config.default_backend != "enigo" {
        return Err(format!(
            "default_backend must be 'win32' or 'enigo', got: {}",
            config.default_backend
        ));
    }

    Ok(())
}

/// 设置 PULL 地址
pub fn set_pull_address(addr: String) -> Result<(), String> {
    let mut config = state::get_config()?;
    config.pull_address = addr;
    validate_config(&config)?;
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
    validate_config(&config)?;
    state::set_config(config)
}

/// 获取 PUB 地址
pub fn get_pub_address() -> Result<String, String> {
    let config = state::get_config()?;
    Ok(config.pub_address)
}
