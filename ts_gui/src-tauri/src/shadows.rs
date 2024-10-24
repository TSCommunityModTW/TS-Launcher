/// Init shadows
#[cfg(target_os = "windows")]
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::Manager;
    use window_shadows::set_shadow;
    let window = app.get_window("main").unwrap();
    window.set_decorations(false).unwrap();
    set_shadow(&window, true).unwrap();
    Ok(())
}