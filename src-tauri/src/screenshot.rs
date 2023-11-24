use screenshots::Screen;
use serde::Deserialize;
use tauri::{api::path::app_data_dir, AppHandle, Manager};
use std::{time::Instant, path::PathBuf, fs};

use crate::overlay::toggle_overlay_window;

fn capture_screen(selection: &SelectionCoords, file_path: &PathBuf) {
    let start = Instant::now();
    let screen = Screen::from_point(0, 0).unwrap();
    println!("capturer {screen:?}");

    let x: i32;
    let y: i32;
    let width: u32;
    let height: u32;
    if selection.target.0 > selection.origin.0 {
        x = selection.origin.0;
        width = (selection.target.0 - selection.origin.0) as u32;
    } else {
        x = selection.target.0;
        width = (selection.origin.0 - selection.target.0) as u32;
    }

    if selection.target.1 > selection.origin.1 {
        y = selection.origin.1;
        height = (selection.target.1 - selection.origin.1) as u32;
    } else {
        y = selection.target.1;
        height = (selection.origin.1 - selection.target.1) as u32;
    }

    // FIXME: crash on 0 width or height
    if (width == 0) || (height == 0) {
        return;
    }
    
    let image = screen.capture_area(x, y, width, height).unwrap();
    // FIXME: save file to proper location
    image.save(file_path).unwrap();
    println!("Runtime: {:?}", start.elapsed());
}

#[derive(Debug, Deserialize)]
pub struct SelectionCoords {
    pub origin: (i32, i32),
    pub target: (i32, i32),
}

fn get_screenshot_path(app_handle: &AppHandle) -> PathBuf {
    let config = app_handle.config();
    // FIXME: show error message to the user
    let mut app_dir = app_data_dir(&config).unwrap();
    app_dir.push("screenshots");

    if !app_dir.exists() {
        match fs::create_dir_all(&app_dir) {
            Ok(_) => println!("Directories created successfully"),
            Err(e) => println!("Error creating directories: {}", e),
        }
    } else {
        println!("Path already exists");
    }
    // generate timestamp screenshot name 
    let screenshot_name = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S.png").to_string();
    app_dir.push(screenshot_name);

    app_dir
}

#[tauri::command]
pub fn screenshot(app_handle: AppHandle, coords: SelectionCoords) {
    let overlay = app_handle.get_window("overlay").unwrap();
    if !overlay.is_visible().unwrap() {
        return;
    }
    
    let screenshot_path = get_screenshot_path(&app_handle);
    println!("app_dir: {:?}", screenshot_path);

    println!("selection coords: {:?}", coords);
    println!("screenshot");
    capture_screen(&coords, &screenshot_path);

    toggle_overlay_window(&app_handle);
}
