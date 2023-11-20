// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, Size, SystemTray, SystemTrayEvent,
    SystemTrayMenu,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const SHORTCUT_CAPTURE: &str = "Command+Ctrl+a";

fn system_tray() -> SystemTray {
    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(
                CustomMenuItem::new("capture".to_string(), "Capture").accelerator(SHORTCUT_CAPTURE),
            )
            .add_item(CustomMenuItem::new("exit".to_string(), "Quit").accelerator("Q")),
    )
}

fn toggle_overlay_window(app: &tauri::AppHandle) {
    let overlay = app.get_window("overlay");
    // if overlay exist
    if let Some(overlay) = overlay {
        // if overlay is visible
        if overlay.is_visible().unwrap() {
            // hide it
            overlay.hide().unwrap();
        } else {
            // show it
            overlay.show().unwrap();
        }
    } else {
        // if overlay not exist create new
        let overlay =
            tauri::WindowBuilder::new(app, "overlay", tauri::WindowUrl::App("overlay.html".into()))
                .resizable(false)
                .transparent(true)
                .decorations(false)
                .position(0.0, 0.0)
                .build()
                .unwrap();

        let monitor = match overlay.current_monitor() {
            Ok(mon) => mon,
            Err(_) => panic!("No monitor found!"),
        }
        .unwrap();

        let physical_size = monitor.size().clone();
        let size = Size::from(physical_size);
        // let size = LogicalSize
        let _ = overlay.set_size(size);

        // overlay.set_size(overlay.current_monitor().size());
        // show it
        overlay.show().unwrap();
    }
}

fn on_system_tray_event<'a>(app: &'a tauri::AppHandle, event: SystemTrayEvent) {
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

fn setup_shortcuts(app: &tauri::App) {
    let app_handle = app.app_handle();
    let mut manager = app.global_shortcut_manager();

    // Clone app_handle for the closure
    let app_handle_clone = app_handle.clone();
    manager
        .register(SHORTCUT_CAPTURE, move || {
            toggle_overlay_window(&app_handle_clone);
        })
        .expect("Failed to register global shortcut");
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            setup_shortcuts(app);
            Ok(())
        })
        .system_tray(system_tray())
        .on_system_tray_event(on_system_tray_event)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
