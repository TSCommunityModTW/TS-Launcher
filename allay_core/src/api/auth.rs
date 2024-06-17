use chrono::{Duration, Local};

use crate::{util::{oauth::{microsoft::{DeviceAuth, self}, minecraft::{self}}, app_path}, Store};

pub async fn get_device_code() -> crate::Result<DeviceAuth> {
    microsoft::auth_device_code().await
}

pub async fn auth_minecraft_await(device_auth: &DeviceAuth) -> crate::Result<bool> {
    
    let microsoft_auth = microsoft::auth_verification_user(&device_auth).await?;

    if let Some(microsoft_token_auth) = microsoft_auth {

        let minecraft_auth = minecraft::auth_minecraft(&microsoft_token_auth.access_token).await?;

        let store = Store::get().await?;
        let mut profiles = store.profiles.write().await;

        profiles.player.name = minecraft_auth.name;
        profiles.player.uuid = minecraft_auth.uuid;
        profiles.microsoft_auth.mc_account_token = minecraft_auth.access_token;

        let expires_at = Local::now() + Duration::seconds(microsoft_token_auth.expires_in as i64);
        profiles.microsoft_auth.expires_at = expires_at.timestamp();

        profiles.set_microsoft_access_token(&microsoft_token_auth.access_token)?;
        profiles.set_microsoft_refresh_token(&microsoft_token_auth.refresh_token)?;

        profiles.sync(&app_path::get_profile_json_file_path()).await?;

        return Ok(true);
    }

    Ok(false)
}

pub async fn auth_verification_expires_at() -> crate::Result<bool> {

    let store = Store::get().await?;
    let mut profiles = store.profiles.write().await;

    let microsoft_expires_at = &profiles.microsoft_auth.expires_at;
    let now = Local::now();
    if *microsoft_expires_at==0{
        tracing::info!("沒有登入記錄!");
    }
    else if now.timestamp() > *microsoft_expires_at {
        
        tracing::warn!("Microsoft 帳號過期，嘗試取得新的 Token!");

        let microsoft_refresh_token = profiles.get_microsoft_refresh_token()?;

        if microsoft_refresh_token.is_empty() {
            return Ok(false);
        }

        let new_microsoft_auth = microsoft::refresh_access_token(&microsoft_refresh_token).await?;
        let new_minecraft_auth = minecraft::auth_minecraft(&new_microsoft_auth.access_token).await?;

        profiles.player.name = new_minecraft_auth.name;
        profiles.player.uuid = new_minecraft_auth.uuid;
        profiles.microsoft_auth.mc_account_token = new_minecraft_auth.access_token;

        let expires_at = Local::now() + Duration::seconds(new_microsoft_auth.expires_in as i64);
        profiles.microsoft_auth.expires_at = expires_at.timestamp();

        profiles.set_microsoft_access_token(&new_microsoft_auth.access_token)?;
        profiles.set_microsoft_refresh_token(&new_microsoft_auth.refresh_token)?;

        profiles.sync(&app_path::get_profile_json_file_path()).await?;

        tracing::info!("Microsoft 嘗試取得新的 Token 帳號驗證成功!");

        return Ok(true);
    }
    else {
        tracing::info!("Microsoft 帳號驗證成功!");
    }

    Ok(true)
}

// pub async fn auth_minecraft(access_token: &str) -> crate::Result<MinecraftAuth> {
//     minecraft::auth_minecraft(&access_token).await
// }