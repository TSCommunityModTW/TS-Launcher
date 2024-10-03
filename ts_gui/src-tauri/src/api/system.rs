use allay_core::prelude::java;
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("system")
        .invoke_handler(tauri::generate_handler![
            get_max_memory_size,
            get_free_memory_size
            // get_used_memory_size
        ])
        .build()
}

// invoke("plugin:system|get_max_memory_size");
#[tauri::command]
pub fn get_max_memory_size() -> super::Result<u64> {
    Ok(java::get_java_max_memory_size()?)
}

// invoke("plugin:system|get_free_memory_size");
#[tauri::command]
pub fn get_free_memory_size() -> super::Result<u64> {
    Ok(java::get_free_memory_size()?)
}
