use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const PROJECT_FILE_NAME: &str = "project.json";
pub const CONFIG_DIR_NAME: &str = ".otacle";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub version: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl ProjectConfig {
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            version: "1.0".to_string(),
            name,
            description,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read project.json: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse project.json: {}", e))
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize project.json: {}", e))?;
        fs::write(path, content).map_err(|e| format!("Failed to write project.json: {}", e))
    }
}

pub fn get_config_dir(project_dir: &Path) -> PathBuf {
    project_dir.join(CONFIG_DIR_NAME)
}

pub fn get_project_config_path(project_dir: &Path) -> PathBuf {
    get_config_dir(project_dir).join(PROJECT_FILE_NAME)
}

pub fn get_actions_config_path(project_dir: &Path) -> PathBuf {
    get_config_dir(project_dir).join("actions.json")
}

pub fn get_observe_config_path(project_dir: &Path) -> PathBuf {
    get_config_dir(project_dir).join("observe.json")
}

pub fn get_comm_config_path(project_dir: &Path) -> PathBuf {
    get_config_dir(project_dir).join("comm.json")
}

pub fn get_think_config_path(project_dir: &Path) -> PathBuf {
    get_config_dir(project_dir).join("think.json")
}

pub fn is_valid_project(path: &Path) -> bool {
    get_project_config_path(path).exists()
}
