use tauri::{App, Manager};
use window_vibrancy::{apply_mica, apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

/// Init vibrancy
pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {

    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, Some(NSVisualEffectState::Active), None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    apply_mica(&window, None).expect("Unsupported platform! 'apply_mica' is only supported on Windows");
    // apply_blur(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    Ok(())
}