use tauri::{Manager, PhysicalPosition};

pub fn toggle_overlay_window(app: &tauri::AppHandle) {
    let overlay = app.get_window("overlay");
    if let Some(overlay) = overlay {
        if overlay.is_visible().unwrap() {
            overlay.hide().unwrap();
        } else {
            overlay.show().unwrap();
        }
    } else {
        let overlay =
            tauri::WindowBuilder::new(app, "overlay", tauri::WindowUrl::App("index.html?window_id=overlay".into()))
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

        let result = overlay.set_focus();
        if let Err(e) = result {
            println!("Error: {}", e);
        }
    }
}
