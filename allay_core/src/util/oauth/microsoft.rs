use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::util::config::MICROSOFT_CLIENT_ID;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceAuth {
    pub device_code: String,
    pub expires_in: u32,
    pub interval: u32,
    pub message: String,
    pub user_code: String,
    pub verification_uri: String
}

#[derive(Debug, Deserialize)]
pub struct TokenAuth {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
    pub ext_expires_in: u32,
    pub scope: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
struct TokenAuthError {
    correlation_id: String,
    error: String,
    error_codes: Vec<u32>,
    error_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_uri: Option<String>,
    timestamp: String,
    trace_id: String,
}

#[tracing::instrument]
pub async fn auth_device_code() -> crate::Result<DeviceAuth> {

    let mut params = HashMap::new();
    params.insert("client_id", MICROSOFT_CLIENT_ID);
    params.insert("scope", "XboxLive.signin,offline_access");

    let device_code_response = Client::new()
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    if device_code_response.status().is_success() {
        let text = device_code_response.text().await?;
        let device_auth_response = serde_json::from_str::<DeviceAuth>(&text)?;

        tracing::info!("Microsoft auth verification uri: {}", device_auth_response.verification_uri);
        tracing::info!("Microsoft auth device code: {}", device_auth_response.user_code);

        return Ok(device_auth_response);
    }

    Err(crate::ErrorKind::MicrosoftAuthError("Microsoft 裝置授權授失敗".to_owned()).as_error())
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenAuth {
    pub access_token: String,
    pub expires_in: u32,
    pub ext_expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String
}

#[tracing::instrument(skip(refresh_token))]
pub async fn refresh_access_token(refresh_token: &str) -> crate::Result<RefreshTokenAuth> {

    let mut params = HashMap::new();
    params.insert("client_id", MICROSOFT_CLIENT_ID);
    params.insert("scope", "XboxLive.signin,offline_access");
    params.insert("refresh_token", refresh_token);
    params.insert("grant_type", "refresh_token");

    let refresh_token_response = Client::new()
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    if refresh_token_response.status().is_success() {
        let text = refresh_token_response.text().await?;
        let refresh_token_auth = serde_json::from_str::<RefreshTokenAuth>(&text)?;
        return Ok(refresh_token_auth);
    }

    Err(crate::ErrorKind::MicrosoftAuthError("Microsoft 重新整理權杖失敗".to_owned()).as_error())
}

#[tracing::instrument(skip(device_auth))]
pub async fn auth_verification_user(device_auth: &DeviceAuth) -> crate::Result<Option<TokenAuth>> {
    loop {
        let mut params = HashMap::new();
        params.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
        params.insert("client_id", MICROSOFT_CLIENT_ID);
        params.insert("device_code", &device_auth.device_code);

        let token_response = Client::new()
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        if token_response.status().is_success() {
            let text = token_response.text().await?;
            let token_auth = serde_json::from_str::<TokenAuth>(&text)?;
            return Ok(Some(token_auth));
        }

        let text = token_response.text().await?;
        let error_token_error = serde_json::from_str::<TokenAuthError>(&text)?; 

        // invalid_request 無效的請求
        // authorization_declined 終端使用者拒絕授權要求。
        // bad_verification_code device_code 無法辨識傳送至 /token 端點的。
        // expired_token expires_in 的值已超過，且不再能夠使用 device_code 進行驗證。
        if error_token_error.error == "authorization_declined" || error_token_error.error == "expired_token" || error_token_error.error == "bad_verification_code" || error_token_error.error == "invalid_request" {
            return Ok(None);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(device_auth.interval.into())).await;
    }
}