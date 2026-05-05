//! 动作配置验证逻辑

use super::types::{ActionData, ActionList, Variable};
use std::collections::HashSet;

/// 验证单个动作
pub fn validate_action(action: &ActionData) -> Result<(), String> {
    match action {
        ActionData::Key(key_action) => {
            if key_action.key.is_empty() {
                return Err("Key name cannot be empty".to_string());
            }
            validate_variables(&key_action.variables)?;
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
            validate_variables(&seq_action.variables)?;
        }
        ActionData::MouseClick(click_action) => {
            if click_action.count == 0 {
                return Err("Click count must be at least 1".to_string());
            }
            validate_variables(&click_action.variables)?;
        }
        ActionData::MouseMove(move_action) => {
            // x, y 可以是任意值 - 无需验证
            validate_variables(&move_action.variables)?;
        }
        ActionData::MouseScroll(scroll_action) => {
            if scroll_action.amount == 0 {
                return Err("Scroll amount must not be zero".to_string());
            }
            validate_variables(&scroll_action.variables)?;
        }
        ActionData::Delay(delay_action) => {
            // duration 可以是任意值 - 允许 0 表示无操作
            validate_variables(&delay_action.variables)?;
        }
        ActionData::Text(text_action) => {
            if text_action.content.is_empty() {
                return Err("Text content cannot be empty".to_string());
            }
            validate_variables(&text_action.variables)?;
        }
    }
    Ok(())
}

/// 验证 variables 中的 param_name 和 field_name 唯一性
fn validate_variables(variables: &[Variable]) -> Result<(), String> {
    let mut seen_params: HashSet<&str> = HashSet::new();
    let mut seen_fields: HashSet<&str> = HashSet::new();

    for var in variables {
        if !var.param_name.is_empty() {
            if !seen_params.insert(&var.param_name) {
                return Err(format!("Duplicate param_name '{}'", var.param_name));
            }
        }
        if !var.field_name.is_empty() {
            if !seen_fields.insert(&var.field_name) {
                return Err(format!("Duplicate field_name '{}'", var.field_name));
            }
        }
    }
    Ok(())
}

/// 验证整个配置
pub fn validate_config(config: &ActionList) -> Result<(), String> {
    // 空配置是有效的起始状态（新项目），不进行检查
    if config.is_empty() {
        return Ok(());
    }

    for (idx, item) in config.iter().enumerate() {
        validate_action(&item.data).map_err(|e| format!("Action {}: {}", idx, e))?;
    }

    Ok(())
}
