use crate::remote::client::{with_client, ApiResponse, AuthResponse, LoginRequest, RegisterRequest};

pub fn register(username: &str, password: &str, email: Option<&str>) -> Result<AuthResponse, String> {
    with_client(|client, base_url| {
        let req = RegisterRequest {
            username: username.to_string(),
            password: password.to_string(),
            email: email.map(|s| s.to_string()),
        };

        let resp = client
            .post(format!("{}/api/auth/register", base_url))
            .json(&req)
            .send()
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }

        let api_resp: ApiResponse<AuthResponse> = resp.json().map_err(|e| e.to_string())?;
        let auth = api_resp.into_result()?;
        crate::remote::client::set_token(auth.token.clone());
        Ok(auth)
    })
}

pub fn login(username: &str, password: &str) -> Result<AuthResponse, String> {
    with_client(|client, base_url| {
        let req = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let resp = client
            .post(format!("{}/api/auth/login", base_url))
            .json(&req)
            .send()
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }

        let api_resp: ApiResponse<AuthResponse> = resp.json().map_err(|e| e.to_string())?;
        let auth = api_resp.into_result()?;
        crate::remote::client::set_token(auth.token.clone());
        Ok(auth)
    })
}

pub fn get_me() -> Result<crate::remote::client::Account, String> {
    with_client(|client, base_url| {
        let token = crate::remote::client::get_token().ok_or("not logged in")?;

        let resp = client
            .get(format!("{}/api/auth/me", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("http error: {}", resp.status()));
        }

        let api_resp: ApiResponse<crate::remote::client::Account> =
            resp.json().map_err(|e| e.to_string())?;
        api_resp.into_result()
    })
}

pub fn logout() {
    crate::remote::client::clear_token();
}

pub fn set_token(token: &str) {
    crate::remote::client::set_token(token.to_string());
}