use tracing::{info, warn, error, debug};
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("logger")
        .invoke_handler(tauri::generate_handler![
            log_message
        ])
        .build()
}

/// await invoke("plugin:logger|log_message", { level, message });
#[tauri::command]
pub async fn log_message(level: String, message: String) {
    match level.as_str() {
        "info" => info!("{}", message),
        "warn" => warn!("{}", message),
        "error" => error!("{}", message),
        "debug" => debug!("{}", message),
        _ => info!("{}", message),
    }
}