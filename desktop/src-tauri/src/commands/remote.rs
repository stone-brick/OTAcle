use tauri::command;
use tauri::AppHandle;
use tauri_store::ManagerExt;

use crate::remote::client::{init, AuthResponse, Account};
use crate::remote::auth as auth_mod;
use crate::remote::group;
use crate::remote::config;
use crate::remote::{Group, GroupMember, ConfigVersion, ConfigDownloadResponse};

const STORE_PATH: &str = "settings.json";
const SERVER_URL_KEY: &str = "remote_server_url";
const DEFAULT_SERVER_URL: &str = "http://localhost:8080";

// === Client Management ===

#[command]
pub fn remote_init(base_url: String) -> Result<(), String> {
    init(&base_url);
    Ok(())
}

#[command]
pub fn remote_is_initialized() -> bool {
    !crate::remote::client::base_url().is_empty()
}

// === Auth ===

#[derive(serde::Serialize)]
pub struct RegisterResponse {
    pub token: String,
    pub user_id: i64,
    pub username: String,
}

impl From<AuthResponse> for RegisterResponse {
    fn from(r: AuthResponse) -> Self {
        Self {
            token: r.token,
            user_id: r.user_id,
            username: r.username,
        }
    }
}

#[command]
pub fn remote_auth_register(
    base_url: String,
    username: String,
    password: String,
    email: Option<String>,
) -> Result<RegisterResponse, String> {
    init(&base_url);
    auth_mod::register(&username, &password, email.as_deref()).map(Into::into)
}

#[command]
pub fn remote_auth_login(
    base_url: String,
    username: String,
    password: String,
) -> Result<RegisterResponse, String> {
    init(&base_url);
    auth_mod::login(&username, &password).map(Into::into)
}

#[command]
pub fn remote_auth_get_me() -> Result<Account, String> {
    auth_mod::get_me()
}

#[command]
pub fn remote_auth_set_token(token: String) {
    auth_mod::set_token(&token);
}

#[command]
pub fn remote_auth_logout() {
    auth_mod::logout();
}

// === Groups ===

#[command]
pub fn remote_group_list() -> Result<Vec<Group>, String> {
    group::list()
}

#[command]
pub fn remote_group_create(
    name: String,
    description: Option<String>,
) -> Result<Group, String> {
    group::create(&name, description.as_deref())
}

#[command]
pub fn remote_group_detail(group_id: i64) -> Result<Group, String> {
    group::detail(group_id)
}

#[command]
pub fn remote_group_update(
    group_id: i64,
    name: String,
    description: Option<String>,
) -> Result<Group, String> {
    group::update(group_id, &name, description.as_deref())
}

#[command]
pub fn remote_group_delete(group_id: i64) -> Result<(), String> {
    group::delete(group_id)
}

#[command]
pub fn remote_group_members(group_id: i64) -> Result<Vec<GroupMember>, String> {
    group::members(group_id)
}

#[command]
pub fn remote_group_join(group_id: i64, invite_code: String) -> Result<(), String> {
    group::join(group_id, &invite_code)
}

#[command]
pub fn remote_group_leave(group_id: i64) -> Result<(), String> {
    group::leave(group_id)
}

// === Config ===

#[command]
pub fn remote_config_upload(
    group_id: i64,
    character_id: String,
    file_path: String,
) -> Result<ConfigVersion, String> {
    config::upload(group_id, &character_id, &file_path)
}

#[command]
pub fn remote_config_download_url(
    group_id: i64,
    character_id: String,
    version: Option<i32>,
) -> Result<ConfigDownloadResponse, String> {
    config::download_url(group_id, &character_id, version)
}

#[command]
pub fn remote_config_latest(
    group_id: i64,
    character_id: String,
) -> Result<ConfigDownloadResponse, String> {
    config::latest(group_id, &character_id)
}

#[command]
pub fn remote_config_versions(
    group_id: i64,
    character_id: String,
) -> Result<Vec<ConfigVersion>, String> {
    config::versions(group_id, &character_id)
}

// === Store ===

#[command]
pub fn remote_save_server_url(url: String, app: AppHandle) -> Result<(), String> {
    app.store_collection()
        .set(STORE_PATH, SERVER_URL_KEY, serde_json::Value::String(url))
        .map_err(|e| e.to_string())?;
    app.store_collection().save(STORE_PATH).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn remote_load_server_url(app: AppHandle) -> Result<String, String> {
    let url = match app
        .store_collection()
        .get::<String>(STORE_PATH, SERVER_URL_KEY)
    {
        Ok(v) => v,
        Err(_) => DEFAULT_SERVER_URL.to_string(),
    };
    Ok(url)
}