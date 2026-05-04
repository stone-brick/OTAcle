use crate::remote::client::{with_client, ApiResponse};

pub fn upload(
    group_id: i64,
    character_id: &str,
    file_path: &str,
) -> Result<crate::remote::ConfigVersion, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;

        let file_data = std::fs::read(file_path).map_err(|e| e.to_string())?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("config.zip")
            .to_string();

        let part = reqwest::blocking::multipart::Part::bytes(file_data)
            .file_name(file_name)
            .mime_str("application/zip")
            .map_err(|e| e.to_string())?;

        let form = reqwest::blocking::multipart::Form::new()
            .text("groupId", group_id.to_string())
            .text("characterId", character_id.to_string())
            .part("file", part);

        let resp = client
            .post(format!("{}/api/configs/upload", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .multipart(form)
            .send()
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }

        let api_resp: ApiResponse<crate::remote::ConfigVersion> = resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}

pub fn download_url(
    group_id: i64,
    character_id: &str,
    version: Option<i32>,
) -> Result<crate::remote::ConfigDownloadResponse, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;

        let mut req = client
            .get(format!("{}/api/configs/download", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .query(&[("groupId", group_id.to_string())])
            .query(&[("characterId", character_id.to_string())]);

        if let Some(v) = version {
            req = req.query(&[("version", v.to_string())]);
        }

        let resp = req.send().map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }

        let api_resp: ApiResponse<crate::remote::ConfigDownloadResponse> =
            resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}

pub fn latest(group_id: i64, character_id: &str) -> Result<crate::remote::ConfigDownloadResponse, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;

        let resp = client
            .get(format!("{}/api/configs/latest", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .query(&[("groupId", group_id.to_string())])
            .query(&[("characterId", character_id.to_string())])
            .send()
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }

        let api_resp: ApiResponse<crate::remote::ConfigDownloadResponse> =
            resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}

pub fn versions(group_id: i64, character_id: &str) -> Result<Vec<crate::remote::ConfigVersion>, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;

        let resp = client
            .get(format!("{}/api/configs/versions", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .query(&[("groupId", group_id.to_string())])
            .query(&[("characterId", character_id.to_string())])
            .send()
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }

        let api_resp: ApiResponse<Vec<crate::remote::ConfigVersion>> =
            resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}