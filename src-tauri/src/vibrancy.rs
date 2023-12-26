use tauri::{App, Manager};
use window_vibrancy::{apply_acrylic, apply_vibrancy, NSVisualEffectMaterial};

/// Init vibrancy
pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {

    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    apply_acrylic(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    Ok(())
}