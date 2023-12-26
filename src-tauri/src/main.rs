// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod vibrancy;

fn main() {

    let mut builder = tauri::Builder::default();

    builder = builder.setup(move |app| {
        vibrancy::init(app)?;
        Ok(())
    });

    let builder = builder
        .plugin(api::auth::init())
        .plugin(api::player::init());

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


}
