//! Configuration loading and saving for Observe module

use std::fs;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::observe::types::ObserveConfig;

lazy_static! {
    /// Global observe configuration storage
    pub static ref OBSERVE_CONFIG: Mutex<Option<ObserveConfig>> = Mutex::new(None);
}

/// Load observe configuration from a JSON file
pub fn load_config(path: &str) -> Result<ObserveConfig, String> {
    if !std::path::Path::new(path).exists() {
        // Return default config if file doesn't exist
        let default_config = ObserveConfig::default();
        return Ok(default_config);
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: ObserveConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    // Store in global state
    let mut global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;
    *global = Some(config.clone());

    Ok(config)
}

/// Save observe configuration to a JSON file
pub fn save_config(path: &str, config: &ObserveConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    // Update global state
    let mut global = OBSERVE_CONFIG.lock()
        .map_err(|_| "Failed to lock observe config".to_string())?;
    *global = Some(config.clone());

    Ok(())
}

