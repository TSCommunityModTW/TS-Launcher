use allay_core::prelude::process;
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("process")
        .invoke_handler(tauri::generate_handler![
            process_minecraft_run
        ])
        .build()
}

/// await invoke("plugin:process|process_minecraft_run", { server_id, children_server_id });
#[tauri::command]
pub async fn process_minecraft_run(server_id: &str, children_server_id: &str) -> super::Result<()> {
    process::process_minecraft_run(server_id, children_server_id).await?;
    Ok(())
}