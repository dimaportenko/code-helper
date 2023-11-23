use screenshots::Screen;
use serde::Deserialize;
use std::time::Instant;

fn capture_screen(selection: &SelectionCoords) {
    let start = Instant::now();
    let _screens = Screen::all().unwrap();

    // for screen in screens {
    //     println!("capturer {screen:?}");
    //     let mut image = screen.capture().unwrap();
    //     image
    //         .save(format!("target/{}.png", screen.display_info.id))
    //         .unwrap();
    //
    //     image = screen.capture_area(300, 300, 300, 300).unwrap();
    //     image
    //         .save(format!("target/{}-2.png", screen.display_info.id))
    //         .unwrap();
    // }
    //
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

    let image = screen.capture_area(x, y, width, height).unwrap();
    image.save("target/capture_display_with_point.png").unwrap();
    println!("Runtime: {:?}", start.elapsed());
}

#[derive(Debug, Deserialize)]
pub struct SelectionCoords {
    pub origin: (i32, i32),
    pub target: (i32, i32),
}

#[tauri::command]
pub fn screenshot(coords: SelectionCoords) {
    println!("selection coords: {:?}", coords);
    println!("screenshot");
    capture_screen(&coords);
}