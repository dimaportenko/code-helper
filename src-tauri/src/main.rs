// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use configuration::{setup_shortcuts, on_system_tray_event, system_tray};
use screenshot::screenshot;

mod configuration;
mod overlay;
mod screenshot;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            setup_shortcuts(app);
            Ok(())
        })
        .system_tray(system_tray())
        .on_system_tray_event(on_system_tray_event)
        .invoke_handler(tauri::generate_handler![greet, screenshot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
