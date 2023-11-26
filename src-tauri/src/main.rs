// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_directory::open_app_directory;
use configuration::{on_system_tray_event, setup_shortcuts, system_tray};
use overlay::stop_screenshot;
use screenshot::{screenshot, get_screenshot_files};

mod app_directory;
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
        .invoke_handler(tauri::generate_handler![
            screenshot,
            stop_screenshot,
            open_app_directory,
            get_screenshot_files
        ])
        .run(tauri::generate_context!())
        .expect("ERROR: error while running tauri application");
}
