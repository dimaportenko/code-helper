use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

use crate::overlay::toggle_overlay_window;
const SHORTCUT_CAPTURE: &str = "Command+Ctrl+a";

pub fn system_tray() -> SystemTray {
    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(
                CustomMenuItem::new("capture".to_string(), "Capture").accelerator(SHORTCUT_CAPTURE),
            )
            .add_item(CustomMenuItem::new("exit".to_string(), "Quit").accelerator("Q")),
    )
}

pub fn on_system_tray_event<'a>(app: &'a tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "capture" => toggle_overlay_window(app),
                "exit" => {
                    // Quit the app
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
}

pub fn setup_shortcuts(app: &tauri::App) {
    let app_handle = app.app_handle();
    let mut manager = app.global_shortcut_manager();

    // Clone app_handle for the closure
    let app_handle_clone = app_handle.clone();
    manager
        .register(SHORTCUT_CAPTURE, move || {
            toggle_overlay_window(&app_handle_clone);
        })
        .expect("ERROR: Failed to register global shortcut");
}
