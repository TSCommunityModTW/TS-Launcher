use allay_core::launcher::LauncherStore;
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("player")
        .invoke_handler(tauri::generate_handler![
            get_player_name,
            get_player_uuid
        ])
        .build()
}

#[tauri::command]
pub async fn get_player_name() -> crate::api::Result<String> {
    let mut launcher_store = LauncherStore::init().await?;
    Ok(launcher_store.player_name().get().to_owned())
}

#[tauri::command]
pub async fn get_player_uuid() -> crate::api::Result<String> {
    let mut launcher_store = LauncherStore::init().await?;
    Ok(launcher_store.player_uuid().get().to_owned())
}

