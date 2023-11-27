

#[tauri::command]
pub fn image_to_code(file_path: String) {
    println!("image_to_code: {:?}", file_path);
}
