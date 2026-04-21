use super::config::{get_config_dir, ProjectConfig};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateManifest {
    pub templates: Vec<TemplateConfig>,
}

pub fn get_templates_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates")
}

pub fn get_default_template_dir() -> PathBuf {
    get_templates_dir().join("default")
}

pub fn list_templates() -> Result<Vec<TemplateConfig>, String> {
    let manifest_path = get_templates_dir().join("manifest.json");
    if !manifest_path.exists() {
        return Ok(vec![TemplateConfig {
            name: "default".to_string(),
            description: "默认模板".to_string(),
        }]);
    }
    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read template manifest: {}", e))?;
    let manifest: TemplateManifest = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse template manifest: {}", e))?;
    Ok(manifest.templates)
}

pub fn copy_template_to_project(
    template_name: &str,
    project_dir: &Path,
    project_name: &str,
) -> Result<(), String> {
    let template_dir = if template_name == "default" {
        get_default_template_dir()
    } else {
        get_templates_dir().join(template_name)
    };

    let template_config_dir = template_dir.join(".otacle");
    if !template_config_dir.exists() {
        return Err(format!("Template '{}' not found", template_name));
    }

    let dest_config_dir = get_config_dir(project_dir);

    fs::create_dir_all(&dest_config_dir)
        .map_err(|e| format!("Failed to create .otacle directory: {}", e))?;

    copy_dir_recursive(&template_config_dir, &dest_config_dir)?;

    let project_config_path = dest_config_dir.join("project.json");
    if project_config_path.exists() {
        let mut config = ProjectConfig::load(&project_config_path)?;
        let now = chrono::Utc::now().to_rfc3339();
        config.name = project_name.to_string();
        config.created_at = now.clone();
        config.updated_at = now;
        config.save(&project_config_path)?;
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), String> {
    if !src.is_dir() {
        return Err(format!("Source is not a directory: {}", src.display()));
    }

    fs::create_dir_all(dest)
        .map_err(|e| format!("Failed to create directory {}: {}", dest.display(), e))?;

    for entry in fs::read_dir(src)
        .map_err(|e| format!("Failed to read directory {}: {}", src.display(), e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)
                .map_err(|e| format!("Failed to copy {}: {}", src_path.display(), e))?;
        }
    }

    Ok(())
}
