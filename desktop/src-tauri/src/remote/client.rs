use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Mutex;

static BASE_URL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("http://localhost:8080")));
static TOKEN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn into_result(self) -> Result<T, String> {
        if self.code == 200 {
            self.data.ok_or_else(|| "data is null".to_string())
        } else {
            Err(self.message)
        }
    }
}

pub fn init(base_url: &str) {
    let mut url = BASE_URL.lock().unwrap();
    *url = base_url.trim_end_matches('/').to_string();
}

pub fn set_token(token: String) {
    let mut t = TOKEN.lock().unwrap();
    *t = Some(token);
}

pub fn clear_token() {
    let mut t = TOKEN.lock().unwrap();
    *t = None;
}

pub fn get_token() -> Option<String> {
    TOKEN.lock().unwrap().clone()
}

pub fn base_url() -> String {
    BASE_URL.lock().unwrap().clone()
}

pub fn with_client<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce(&reqwest::blocking::Client, &str) -> Result<R, String>,
{
    let url = BASE_URL.lock().unwrap().clone();
    let client = reqwest::blocking::Client::new();
    f(&client, &url)
}

// Types for sending TO the frontend (Serialize needed)
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Account {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(serde::Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct AuthResponse {
    pub token: String,
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub username: String,
}

// Group types
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "inviteCode")]
    pub invite_code: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct GroupMember {
    pub id: i64,
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    pub role: String,
    #[serde(rename = "joinedAt")]
    pub joined_at: Option<String>,
}

// Config types
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ConfigVersion {
    pub id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "uploaderId")]
    pub uploader_id: i64,
    #[serde(rename = "characterId")]
    pub character_id: String,
    pub version: i32,
    #[serde(rename = "filePath")]
    pub file_path: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSize")]
    pub file_size: Option<i64>,
    pub checksum: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ConfigDownloadResponse {
    #[serde(rename = "configId")]
    pub config_id: i64,
    #[serde(rename = "characterId")]
    pub character_id: String,
    pub version: i32,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    pub checksum: String,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}