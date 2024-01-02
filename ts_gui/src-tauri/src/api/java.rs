use std::path::Path;

use allay_core::java;
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
    java::install_jar(8).await?;
    java::install_jar(16).await?;
    java::install_jar(17).await?;
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