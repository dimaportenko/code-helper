use tauri::{Manager, Size};

pub fn toggle_overlay_window(app: &tauri::AppHandle) {
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
        
        let result = overlay.set_focus();
        if let Err(e) = result {
            println!("Error: {}", e);
        }

    }
}
