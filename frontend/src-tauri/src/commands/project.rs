use crate::project as project_module;
use crate::project::{ProjectInfo, RecentProject};
use std::path::Path;
use tauri::command;

#[command]
pub fn project_open(path: String) -> Result<ProjectInfo, String> {
    let project_path = Path::new(&path);

    if !project_module::is_valid_project(project_path) {
        return Err("Invalid project: project.json not found in .otacle directory".to_string());
    }

    let config = project_module::ProjectConfig::load(&project_module::get_project_config_path(
        project_path,
    ))?;

    let project_info = ProjectInfo::from_config(&config, &path);

    project_module::set_current_project_dir(Some(project_path.to_path_buf()));
    project_module::add_recent_project(path, config.name.clone())?;

    Ok(project_info)
}

#[command]
pub fn project_create(
    path: String,
    name: String,
    template: Option<String>,
) -> Result<ProjectInfo, String> {
    let project_path = Path::new(&path);
    let template_name = template.unwrap_or_else(|| "default".to_string());

    project_module::copy_template_to_project(&template_name, project_path, &name)?;

    let config = project_module::ProjectConfig::load(&project_module::get_project_config_path(
        project_path,
    ))?;

    let project_info = ProjectInfo::from_config(&config, &path);

    project_module::set_current_project_dir(Some(project_path.to_path_buf()));
    project_module::add_recent_project(path, config.name.clone())?;

    Ok(project_info)
}

#[command]
pub fn project_close() -> Result<(), String> {
    project_module::set_current_project_dir(None);
    Ok(())
}

#[command]
pub fn project_get_current() -> Option<ProjectInfo> {
    project_module::get_current_project_dir().and_then(|dir| {
        let config_path = project_module::get_project_config_path(&dir);
        if config_path.exists() {
            project_module::ProjectConfig::load(&config_path)
                .ok()
                .map(|config| ProjectInfo::from_config(&config, &dir.to_string_lossy()))
        } else {
            None
        }
    })
}

#[command]
pub fn project_get_recent() -> Vec<RecentProject> {
    project_module::get_recent_projects()
}

#[command]
pub fn project_remove_recent(path: String) -> Result<(), String> {
    project_module::remove_recent_project(&path)
}

#[command]
pub fn project_toggle_pin(path: String) -> Result<(), String> {
    project_module::toggle_pin_project(&path)
}

#[command]
pub fn project_list_templates() -> Result<Vec<crate::project::template::TemplateConfig>, String> {
    project_module::list_templates()
}

#[command]
pub fn project_get_config_dir() -> Option<String> {
    project_module::get_current_project_dir().map(|p| {
        project_module::get_config_dir(&p)
            .to_string_lossy()
            .to_string()
    })
}

#[command]
pub fn project_get_actions_config_path() -> Option<String> {
    project_module::get_current_project_dir().map(|p| {
        project_module::get_actions_config_path(&p)
            .to_string_lossy()
            .to_string()
    })
}

#[command]
pub fn project_get_observe_config_path() -> Option<String> {
    project_module::get_current_project_dir().map(|p| {
        project_module::get_observe_config_path(&p)
            .to_string_lossy()
            .to_string()
    })
}

#[command]
pub fn project_get_comm_config_path() -> Option<String> {
    project_module::get_current_project_dir().map(|p| {
        project_module::get_comm_config_path(&p)
            .to_string_lossy()
            .to_string()
    })
}
