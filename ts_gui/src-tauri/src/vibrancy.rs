use tauri::{App, Manager};
use window_vibrancy::{apply_vibrancy, clear_mica, NSVisualEffectMaterial, NSVisualEffectState};

/// Init vibrancy
pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {

    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, Some(NSVisualEffectState::Active), None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    clear_mica(&window).expect("Unsupported platform! 'clear_mica' is only supported on Windows");

    Ok(())
}