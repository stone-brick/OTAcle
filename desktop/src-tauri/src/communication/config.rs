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
    let _ = load_config(path);
    Ok(())
}

/// 获取当前配置
pub fn get_config() -> Result<CommConfig, String> {
    state::get_config()
}

/// 验证 CommConfig 的合法性
pub fn validate_config(config: &CommConfig) -> Result<(), String> {
    // 验证 act_pull_address 格式
    if config.act_pull_address.is_empty() {
        return Err("act_pull_address cannot be empty".to_string());
    }
    if !config.act_pull_address.starts_with("tcp://")
        && !config.act_pull_address.starts_with("ipc://")
        && !config.act_pull_address.starts_with("inproc://")
    {
        return Err(format!(
            "act_pull_address must be a valid ZMQ address, got: {}",
            config.act_pull_address
        ));
    }

    // 验证 observe_pub_address 格式
    if config.observe_pub_address.is_empty() {
        return Err("observe_pub_address cannot be empty".to_string());
    }
    if !config.observe_pub_address.starts_with("tcp://")
        && !config.observe_pub_address.starts_with("ipc://")
        && !config.observe_pub_address.starts_with("inproc://")
    {
        return Err(format!(
            "observe_pub_address must be a valid ZMQ address, got: {}",
            config.observe_pub_address
        ));
    }

    // 验证 think_pull_address 格式
    if config.think_pull_address.is_empty() {
        return Err("think_pull_address cannot be empty".to_string());
    }
    if !config.think_pull_address.starts_with("tcp://")
        && !config.think_pull_address.starts_with("ipc://")
        && !config.think_pull_address.starts_with("inproc://")
    {
        return Err(format!(
            "think_pull_address must be a valid ZMQ address, got: {}",
            config.think_pull_address
        ));
    }

    Ok(())
}

/// 设置 Act PULL 地址
pub fn set_act_pull_address(addr: String) -> Result<(), String> {
    let mut config = state::get_config()?;
    config.act_pull_address = addr;
    validate_config(&config)?;
    state::set_config(config)
}

/// 获取 Act PULL 地址
pub fn get_act_pull_address() -> Result<String, String> {
    let config = state::get_config()?;
    Ok(config.act_pull_address)
}

/// 设置 Observe PUB 地址
pub fn set_observe_pub_address(addr: String) -> Result<(), String> {
    let mut config = state::get_config()?;
    config.observe_pub_address = addr;
    validate_config(&config)?;
    state::set_config(config)
}

/// 获取 Observe PUB 地址
pub fn get_observe_pub_address() -> Result<String, String> {
    let config = state::get_config()?;
    Ok(config.observe_pub_address)
}

/// 设置 Think PULL 地址
pub fn set_think_pull_address(addr: String) -> Result<(), String> {
    let mut config = state::get_config()?;
    config.think_pull_address = addr;
    validate_config(&config)?;
    state::set_config(config)
}

/// 获取 Think PULL 地址
pub fn get_think_pull_address() -> Result<String, String> {
    let config = state::get_config()?;
    Ok(config.think_pull_address)
}
