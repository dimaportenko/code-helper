use tauri::{AppHandle, Manager, PhysicalPosition, Window};

fn toggle_window(overlay: &Window) {
    if overlay.is_visible().unwrap() {
        overlay.hide().unwrap();
    } else {
        overlay.show().unwrap();
        overlay.set_focus().unwrap();
    }
}

fn create_overlay_window(app: &AppHandle) {
    let overlay = tauri::WindowBuilder::new(
        app,
        "overlay",
        tauri::WindowUrl::App("index.html?window_id=overlay".into()),
    )
    .always_on_top(true)
    .resizable(false)
    .transparent(true)
    .decorations(false)
    .build()
    .unwrap();

    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSMainMenuWindowLevel, NSWindow};
        use cocoa::base::id;
        let ns_win = overlay.ns_window().unwrap() as id;
        unsafe {
            // kCGMainMenuWindowLevelKey: NSInteger = 8;
            // kCGScreenSaverWindowLevelKey: NSInteger = 13;
            // 13 - 8 = 5
            ns_win.setLevel_(((NSMainMenuWindowLevel + 5) as u64).try_into().unwrap());
        }
    }

    let pos = PhysicalPosition::new(0.0, 0.0);
    let _ = overlay.set_position(pos);

    let monitor = match overlay.current_monitor() {
        Ok(mon) => mon,
        Err(_) => panic!("No monitor found!"),
    }
    .unwrap();

    let physical_size = monitor.size();
    let _ = overlay.set_size(*physical_size);

    overlay.show().unwrap();

    // let hide_result = overlay.set_cursor_visible(false);
    // if let Err(e) = hide_result {
    //     println!("Error: {}", e);
    // }
    let result = overlay.set_focus();
    if let Err(e) = result {
        println!("Error: {}", e);
    }
}

// FIXME: crash on click
pub fn toggle_overlay_window(app: &AppHandle) {
    let main = app.get_window("main").unwrap();
    let overlay = app.get_window("overlay");
    if let Some(overlay) = overlay {
        if overlay.is_visible().unwrap() {
            main.show().unwrap();
        } else {
            main.hide().unwrap();
        }
        toggle_window(&overlay);
    } else {
        main.hide().unwrap();
        create_overlay_window(app);
    }
}
