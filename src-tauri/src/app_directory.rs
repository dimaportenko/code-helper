use std::process::Command as ProcessCommand;
use tauri::{api::path::app_data_dir, command, AppHandle};

#[command]
pub fn open_app_directory(app: AppHandle, subdirectory: Option<String>) {
    let config = app.config();
    let mut path = app_data_dir(&config).expect("Can't get app directory");

    if let Some(subdirectory) = subdirectory {
        path.push(subdirectory);
    }

    if path.is_dir() {
        // Open the directory using the system's default file explorer
        #[cfg(target_os = "windows")]
        ProcessCommand::new("explorer")
            .arg(path)
            .spawn()
            .expect("Failed to open directory");

        #[cfg(target_os = "macos")]
        ProcessCommand::new("open")
            .arg(path)
            .spawn()
            .expect("Failed to open directory");

        #[cfg(target_os = "linux")]
        ProcessCommand::new("xdg-open")
            .arg(path)
            .spawn()
            .expect("Failed to open directory");
    }
}
