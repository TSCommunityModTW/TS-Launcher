use std::path::Path;

use allay_core::{java, settings, Store};
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("java")
        .invoke_handler(tauri::generate_handler![
            auto_install_all_java,
            test_jar
        ])
        .build()
}

/// await invoke("plugin:java|auto_install_all_java");
#[tauri::command]
pub async fn auto_install_all_java() -> super::Result<()> {
    let mut java_settings = settings::get_java("global").await?;
    java_settings.java8_path = java::install_jar(8).await?.to_string_lossy().to_string();
    java_settings.java16_path = java::install_jar(16).await?.to_string_lossy().to_string();
    java_settings.java17_path = java::install_jar(17).await?.to_string_lossy().to_string();
    settings::set_java("global", java_settings).await?;
    Store::sync().await?;
    Ok(())
}

/// await invoke("plugin:java|test_jar", { path });
#[tauri::command]
pub async fn test_jar(path: &str) -> super::Result<bool> {

    if path.is_empty() {
        return Err(JavaInvokeError::PathNotNull().into());
    }

    Ok(java::test_jar(Path::new(path)).await?)
}

#[derive(Debug, thiserror::Error)]
pub enum JavaInvokeError {

    #[error("Test jar path not null")]
    PathNotNull()
}