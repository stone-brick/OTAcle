//! Think 模块配置管理

use std::fs;
use std::sync::atomic::Ordering;
use std::time::Instant;

use super::state::{DECISION_LOGS, RECEIVED_COUNT, PULL_STATE, THINK_CONFIG, UPTIME_START};
use super::types::{DecisionLog, ThinkConfig, ThinkStatus};

/// 加载配置文件
pub fn load_config(path: &str) -> Result<ThinkConfig, String> {
    if !std::path::Path::new(path).exists() {
        let default_config = ThinkConfig::default();
        set_config(default_config.clone())?;
        return Ok(default_config);
    }
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read config: {}", e))?;
    let config: ThinkConfig =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))?;
    validate_config(&config)?;
    let mut think_config = THINK_CONFIG.lock().map_err(|e| e.to_string())?;
    *think_config = Some(config.clone());
    Ok(config)
}

/// 保存配置文件
pub fn save_config(path: &str, config: &ThinkConfig) -> Result<(), String> {
    validate_config(config)?;
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(path, content).map_err(|e| format!("Failed to write config: {}", e))?;
    let _ = load_config(path);
    Ok(())
}

/// 获取当前配置
pub fn get_config() -> Result<ThinkConfig, String> {
    let think_config = THINK_CONFIG.lock().map_err(|e| e.to_string())?;
    think_config
        .clone()
        .ok_or_else(|| "Think config not loaded".to_string())
}

/// 设置配置
pub fn set_config(config: ThinkConfig) -> Result<(), String> {
    validate_config(&config)?;
    let mut think_config = THINK_CONFIG.lock().map_err(|e| e.to_string())?;
    *think_config = Some(config);
    Ok(())
}

/// 添加决策日志
pub fn add_log(log: DecisionLog) -> Result<(), String> {
    let config = get_config()?;
    let max_points = config.max_data_points as usize;

    let mut logs = DECISION_LOGS.lock().map_err(|e| e.to_string())?;
    if logs.len() >= max_points {
        logs.remove(0);
    }
    logs.push(log);
    RECEIVED_COUNT.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

/// 获取决策日志
pub fn get_logs(limit: Option<usize>) -> Result<Vec<DecisionLog>, String> {
    let logs = DECISION_LOGS.lock().map_err(|e| e.to_string())?;
    match limit {
        Some(n) => Ok(logs.iter().rev().take(n).cloned().collect()),
        None => Ok(logs.clone()),
    }
}

/// 获取状态
pub fn get_status() -> ThinkStatus {
    let logs_count = DECISION_LOGS
        .lock()
        .map(|l| l.len())
        .unwrap_or(0) as u64;

    let running = match PULL_STATE.lock() {
        Ok(guard) => guard
            .as_ref()
            .map(|s| s.running.load(Ordering::SeqCst))
            .unwrap_or(false),
        Err(_) => false,
    };

    let uptime = UPTIME_START
        .lock()
        .ok()
        .and_then(|s| s.map(|i| i.elapsed().as_secs()))
        .unwrap_or(0);

    ThinkStatus {
        running,
        logs_count,
        connected: running,
        messages_received: RECEIVED_COUNT.load(Ordering::SeqCst),
        uptime_seconds: uptime,
    }
}

/// 验证配置
fn validate_config(config: &ThinkConfig) -> Result<(), String> {
    if config.max_data_points == 0 {
        return Err("max_data_points must be greater than 0".to_string());
    }
    let mut names = std::collections::HashSet::new();
    for field in &config.display_fields {
        if field.name.is_empty() {
            return Err("display_fields.name cannot be empty".to_string());
        }
        if field.title.is_empty() {
            return Err("display_fields.title cannot be empty".to_string());
        }
        if !names.insert(&field.name) {
            return Err(format!("duplicate display_fields.name: {}", field.name));
        }
    }
    Ok(())
}

/// 设置启动时间
pub fn set_uptime_start() {
    if let Ok(mut uptime) = UPTIME_START.lock() {
        if uptime.is_none() {
            *uptime = Some(Instant::now());
        }
    }
}

/// 重置状态（用于 stop 时）
pub fn reset_state() {
    RECEIVED_COUNT.store(0, Ordering::SeqCst);
    if let Ok(mut uptime) = UPTIME_START.lock() {
        *uptime = None;
    }
    if let Ok(mut logs) = DECISION_LOGS.lock() {
        logs.clear();
    }
}