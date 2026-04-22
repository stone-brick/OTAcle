use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    pub static ref CURRENT_PROJECT_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub path: String,
    pub name: String,
    pub description: Option<String>,
}

pub fn get_current_project_dir() -> Option<PathBuf> {
    CURRENT_PROJECT_DIR.lock().unwrap().clone()
}

pub fn set_current_project_dir(path: Option<PathBuf>) {
    *CURRENT_PROJECT_DIR.lock().unwrap() = path;
}

impl ProjectInfo {
    pub fn from_config(config: &super::config::ProjectConfig, path: &str) -> Self {
        Self {
            path: path.to_string(),
            name: config.name.clone(),
            description: config.description.clone(),
        }
    }
}
