
use allay_core::{data::LauncherAssets, prelude::launcher, launcher_assets::{Server, ServerChildren}};
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("launcher")
        .invoke_handler(tauri::generate_handler![
            get_assets_info,
            get_assets_server,
            get_assets_servers,
            get_assets_children_server
        ])
        .build()
}

/// await invoke("plugin:launcher|get_assets_info");
#[tauri::command]
pub fn get_assets_info() -> super::Result<LauncherAssets> {
    Ok(launcher::get()?)
}

/// await invoke("plugin:launcher|get_assets_servers");
#[tauri::command]
pub fn get_assets_servers() -> super::Result<Vec<Server>> {
    Ok(launcher::get_servers()?)
}

/// await invoke("plugin:launcher|get_assets_server");
#[tauri::command]
pub fn get_assets_server(id: &str) -> super::Result<Server> {
    Ok(launcher::get_server(id)?)
}

/// await invoke("plugin:launcher|get_assets_children_server");
#[tauri::command]
pub fn get_assets_children_server(main_server_id: &str, id: &str) -> super::Result<ServerChildren> {
    Ok(launcher::get_children_server(main_server_id, id)?)
}