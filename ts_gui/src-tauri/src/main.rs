// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod api;
mod vibrancy;
mod shadows;

fn main() {

    let _logger_guard = allay_core::init_logger();

    let mut builder = tauri::Builder::default();

    builder = builder.setup(move |app| {
        vibrancy::init(app)?;
        #[cfg(target_os = "windows")]
        shadows::init(app)?;
        Ok(())
    });

    let builder = builder
        .plugin(api::auth::init())
        .plugin(api::store::init())
        .plugin(api::java::init())
        .plugin(api::system::init())
        .plugin(api::launcher::init())
        .plugin(api::process::init())
        .plugin(api::logger::init())
        .invoke_handler(tauri::generate_handler![
            initialize
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn initialize(app: tauri::AppHandle) -> crate::api::Result<()> {
    allay_core::EventState::initialize(app).await?;
    allay_core::prelude::launcher::initialize_assets().await?;
    Ok(())
}