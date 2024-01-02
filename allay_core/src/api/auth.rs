use crate::{util::{oauth::{microsoft::{DeviceAuth, self}, minecraft::{self}}, app_path}, Store};

pub async fn get_device_code() -> crate::Result<DeviceAuth> {
    microsoft::auth_device_code().await
}

pub async fn auth_minecraft_await(device_auth: &DeviceAuth) -> crate::Result<bool> {
    
    let microsoft_token_auth = microsoft::auth_verification_user(&device_auth).await?;

    if let Some(microsoft_token_auth) = microsoft_token_auth {

        let minecraft_auth = minecraft::auth_minecraft(&microsoft_token_auth.access_token).await?;

        let store = Store::get().await?;
        let mut profiles = store.profiles.write().await;

        profiles.player.name = minecraft_auth.name;
        profiles.player.uuid = minecraft_auth.uuid;
        profiles.microsoft_auth.mc_account_token = minecraft_auth.access_token;

        profiles.set_microsoft_access_token(&microsoft_token_auth.access_token)?;
        profiles.set_microsoft_refresh_token(&microsoft_token_auth.refresh_token)?;

        profiles.sync(&app_path::get_profile_json_file_path()).await?;

        return Ok(true);
    }

    Ok(false)
}

// pub async fn auth_minecraft(access_token: &str) -> crate::Result<MinecraftAuth> {
//     minecraft::auth_minecraft(&access_token).await
// }