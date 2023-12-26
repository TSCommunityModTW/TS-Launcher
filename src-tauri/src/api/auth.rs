use allay_core::{oauth::{microsoft::{self, DeviceAuth}, minecraft}, launcher::LauncherStore};
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("auth")
        .invoke_handler(tauri::generate_handler![
            device_auth,
            auth_verification_await
        ])
        .build()
}

#[tauri::command]
pub async fn device_auth() -> crate::api::Result<DeviceAuth> {
    Ok(microsoft::auth_device_code().await?)
}

#[tauri::command]
pub async fn auth_verification_await(device_auth: DeviceAuth) -> crate::api::Result<()> {

    let microsoft_token_auth = microsoft::auth_verification_user(&device_auth).await?;
    let minecraft_auth = minecraft::auth_minecraft(&microsoft_token_auth.access_token).await?;

    // let mut launcher_store = LauncherStore::init().await?;

    // launcher_store.set_microsoft_access_token(&minecraft_auth.access_token)?;
    // launcher_store.set_microsoft_refresh_token(&microsoft_token_auth.refresh_token)?;
    // launcher_store.player_name().set(minecraft_auth.name);
    // launcher_store.player_uuid().set(minecraft_auth.uuid);
    // launcher_store.save().await?;

    Ok(())
}