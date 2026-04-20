use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use chrono::Utc;
use lazy_static::lazy_static;
use std::sync::Mutex;

const RECENT_FILE_NAME: &str = "recent_projects.json";
const MAX_RECENT_COUNT: usize = 10;

lazy_static! {
    static ref RECENT_PROJECTS: Mutex<Vec<RecentProject>> = Mutex::new(Vec::new());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentProject {
    pub path: String,
    pub name: String,
    pub last_opened: String,
    pub pinned: bool,
}

impl RecentProject {
    pub fn new(path: String, name: String) -> Self {
        Self {
            path,
            name,
            last_opened: Utc::now().to_rfc3339(),
            pinned: false,
        }
    }
}

fn get_recent_file_path() -> Result<PathBuf, String> {
    let app_dir = dirs::data_local_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    Ok(app_dir.join("OTAcle").join(RECENT_FILE_NAME))
}

pub fn load_recent_projects() -> Result<(), String> {
    let path = get_recent_file_path()?;
    if !path.exists() {
        return Ok(());
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read recent projects: {}", e))?;
    let projects: Vec<RecentProject> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse recent projects: {}", e))?;
    *RECENT_PROJECTS.lock().unwrap() = projects;
    Ok(())
}

pub fn save_recent_projects() -> Result<(), String> {
    let path = get_recent_file_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }
    let projects = RECENT_PROJECTS.lock().unwrap();
    let content = serde_json::to_string_pretty(&*projects)
        .map_err(|e| format!("Failed to serialize recent projects: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write recent projects: {}", e))
}

pub fn get_recent_projects() -> Vec<RecentProject> {
    RECENT_PROJECTS.lock().unwrap().clone()
}

pub fn add_recent_project(path: String, name: String) -> Result<(), String> {
    let mut projects = RECENT_PROJECTS.lock().unwrap();

    if let Some(existing) = projects.iter_mut().find(|p| p.path == path) {
        existing.last_opened = Utc::now().to_rfc3339();
    } else {
        projects.insert(0, RecentProject::new(path.clone(), name));
    }

    projects.sort_by(|a, b| {
        if a.pinned != b.pinned {
            b.pinned.cmp(&a.pinned)
        } else {
            b.last_opened.cmp(&a.last_opened)
        }
    });

    if projects.len() > MAX_RECENT_COUNT {
        let pinned_count = projects.iter().filter(|p| p.pinned).count();
        let keep_pinned_count = pinned_count.min(MAX_RECENT_COUNT);
        let keep_unpinned_count = MAX_RECENT_COUNT - keep_pinned_count;

        let mut kept = Vec::new();
        for p in projects.iter() {
            if p.pinned && kept.len() < keep_pinned_count {
                kept.push(p.clone());
            } else if !p.pinned && kept.len() < MAX_RECENT_COUNT && kept.len() < keep_pinned_count + keep_unpinned_count {
                kept.push(p.clone());
            }
        }
        *projects = kept;
    }

    drop(projects);
    save_recent_projects()
}

pub fn remove_recent_project(path: &str) -> Result<(), String> {
    let mut projects = RECENT_PROJECTS.lock().unwrap();
    projects.retain(|p| p.path != path);
    drop(projects);
    save_recent_projects()
}

pub fn toggle_pin_project(path: &str) -> Result<(), String> {
    let mut projects = RECENT_PROJECTS.lock().unwrap();
    if let Some(project) = projects.iter_mut().find(|p| p.path == path) {
        project.pinned = !project.pinned;
    }
    drop(projects);
    save_recent_projects()
}
