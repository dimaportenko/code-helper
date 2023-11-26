use std::{path::PathBuf, process::Command as ProcessCommand};
use tauri::{api::path::app_data_dir, command, AppHandle};

pub fn get_app_dirctory(app: &AppHandle, subdirectory: Option<String>) -> Option<PathBuf> {
    let config = app.config();
    let path = app_data_dir(&config);

    if None == path {
        return None;
    }

    let mut path = path.unwrap();
    if let Some(subdirectory) = subdirectory {
        path.push(subdirectory);
    }

    Some(path)
}

#[command]
pub fn open_app_directory(app: AppHandle, subdirectory: Option<String>) {
    let path = get_app_dirctory(&app, subdirectory).expect("Error: can't get app directory");
    if path.is_dir() {
        // Open the directory using the system's default file explorer
        #[cfg(target_os = "windows")]
        ProcessCommand::new("explorer")
            .arg(path)
            .spawn()
            .expect("ERROR: Failed to open directory");

        #[cfg(target_os = "macos")]
        ProcessCommand::new("open")
            .arg(path)
            .spawn()
            .expect("ERROR: Failed to open directory");

        #[cfg(target_os = "linux")]
        ProcessCommand::new("xdg-open")
            .arg(path)
            .spawn()
            .expect("ERROR: Failed to open directory");
    }
}
