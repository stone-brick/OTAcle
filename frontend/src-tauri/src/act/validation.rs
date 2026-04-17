//! 动作配置验证逻辑

use super::types::{ActionData, ActionList};

/// 验证单个动作
pub fn validate_action(action: &ActionData) -> Result<(), String> {
    match action {
        ActionData::Key(key_action) => {
            if key_action.key.is_empty() {
                return Err("Key name cannot be empty".to_string());
            }
        }
        ActionData::KeySequence(seq_action) => {
            if seq_action.keys.is_empty() {
                return Err("Key sequence cannot be empty".to_string());
            }
            for item in &seq_action.keys {
                if item.key.is_empty() {
                    return Err("Key in sequence cannot be empty".to_string());
                }
            }
        }
        ActionData::MouseClick(click_action) => {
            if click_action.count == 0 {
                return Err("Click count must be at least 1".to_string());
            }
        }
        ActionData::MouseMove(_) => {
            // x, y 可以是任意值 - 无需验证
        }
        ActionData::MouseScroll(scroll_action) => {
            if scroll_action.amount == 0 {
                return Err("Scroll amount must not be zero".to_string());
            }
        }
        ActionData::Delay(_delay_action) => {
            // duration 可以是任意值 - 允许 0 表示无操作
        }
        ActionData::Text(text_action) => {
            if text_action.content.is_empty() {
                return Err("Text content cannot be empty".to_string());
            }
        }
    }
    Ok(())
}

/// 验证整个配置
pub fn validate_config(config: &ActionList) -> Result<(), String> {
    if config.is_empty() {
        return Err("Configuration is empty".to_string());
    }

    for (idx, item) in config.iter().enumerate() {
        validate_action(&item.data).map_err(|e| format!("Action {}: {}", idx, e))?;
    }

    Ok(())
}
