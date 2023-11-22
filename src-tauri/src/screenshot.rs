use screenshots::Screen;
use std::time::Instant;

fn capture_screen() {
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

    let image = screen.capture_area(0, 0, 300, 300).unwrap();
    image.save("target/capture_display_with_point.png").unwrap();
    println!("Runtime: {:?}", start.elapsed());
}

#[tauri::command]
pub fn screenshot() {
    println!("screenshot");
    capture_screen();
}
