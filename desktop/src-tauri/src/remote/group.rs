use serde::Serialize;

use crate::remote::client::{with_client, ApiResponse};

#[derive(Serialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateGroupRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct JoinGroupRequest {
    pub invite_code: String,
}

pub fn list() -> Result<Vec<crate::remote::Group>, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;
        let resp = client
            .get(format!("{}/api/groups", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }
        let api_resp: ApiResponse<Vec<crate::remote::Group>> = resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}

pub fn create(name: &str, description: Option<&str>) -> Result<crate::remote::Group, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;
        let req = CreateGroupRequest {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
        };
        let resp = client
            .post(format!("{}/api/groups", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&req)
            .send()
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }
        let api_resp: ApiResponse<crate::remote::Group> = resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}

pub fn detail(group_id: i64) -> Result<crate::remote::Group, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;
        let resp = client
            .get(format!("{}/api/groups/{}", base_url, group_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }
        let api_resp: ApiResponse<crate::remote::Group> = resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}

pub fn update(group_id: i64, name: &str, description: Option<&str>) -> Result<crate::remote::Group, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;
        let req = UpdateGroupRequest {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
        };
        let resp = client
            .put(format!("{}/api/groups/{}", base_url, group_id))
            .header("Authorization", format!("Bearer {}", token))
            .json(&req)
            .send()
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }
        let api_resp: ApiResponse<crate::remote::Group> = resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}

pub fn delete(group_id: i64) -> Result<(), String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;
        let resp = client
            .delete(format!("{}/api/groups/{}", base_url, group_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }
        let _: ApiResponse<()> = resp.json().map_err(|e| e.to_string())?;
        Ok(())
    })
}

pub fn members(group_id: i64) -> Result<Vec<crate::remote::GroupMember>, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;
        let resp = client
            .get(format!("{}/api/groups/{}/members", base_url, group_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }
        let api_resp: ApiResponse<Vec<crate::remote::GroupMember>> = resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}

pub fn join(group_id: i64, invite_code: &str) -> Result<(), String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;
        let req = JoinGroupRequest {
            invite_code: invite_code.to_string(),
        };
        let resp = client
            .post(format!("{}/api/groups/{}/join", base_url, group_id))
            .header("Authorization", format!("Bearer {}", token))
            .json(&req)
            .send()
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }
        let _: ApiResponse<()> = resp.json().map_err(|e| e.to_string())?;
        Ok(())
    })
}

pub fn leave(group_id: i64) -> Result<(), String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;
        let resp = client
            .delete(format!("{}/api/groups/{}/leave", base_url, group_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }
        let _: ApiResponse<()> = resp.json().map_err(|e| e.to_string())?;
        Ok(())
    })
}