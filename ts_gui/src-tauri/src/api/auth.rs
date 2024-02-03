use allay_core::{auth, data::DeviceAuth};
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("auth")
        .invoke_handler(tauri::generate_handler![
            get_device_code,
            auth_minecraft_await,
            auth_verification_expires_at
        ])
        .build()
}

/// await invoke("plugin:auth|get_device_code");
#[tauri::command]
pub async fn get_device_code() -> crate::api::Result<DeviceAuth> {
    Ok(auth::get_device_code().await?)
}


/// await invoke("plugin:auth|auth_minecraft_await");
#[tauri::command]
pub async fn auth_minecraft_await(device_auth: DeviceAuth) -> crate::api::Result<bool> {
    Ok(auth::auth_minecraft_await(&device_auth).await?)
}

/// await invoke("plugin:auth|auth_verification_expires_at");
#[tauri::command]
pub async fn auth_verification_expires_at() -> super::Result<bool> {
    Ok(auth::auth_verification_expires_at().await?)
}