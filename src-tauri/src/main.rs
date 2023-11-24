// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use configuration::{setup_shortcuts, on_system_tray_event, system_tray};
use screenshot::screenshot;

mod configuration;
mod overlay;
mod screenshot;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            setup_shortcuts(app);
            Ok(())
        })
        .system_tray(system_tray())
        .on_system_tray_event(on_system_tray_event)
        .invoke_handler(tauri::generate_handler![screenshot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
