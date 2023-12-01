mod api;

use crate::utils::file::encode_image_to_base64;
use crate::utils::temp::OPEN_API_KEY;

const OPEN_API_PROMPT: &str = "You are a tailwind css expert and an amazing html/css developer who tries to absolutely match the code with the designs. This is an image of a UI component design. Make sure you try to match the design as shown in the image using tailwind css. Replace images and icons in the component with rounded box or rectangles. Please provide output as plain valid HTML only without using backticks or labeling it as HTML or any extra text or any description.";

#[tauri::command]
pub async fn image_to_code(file_path: String) -> Result<String, String> {
    println!("image_to_code: {:?}", file_path);

    let base64_image = encode_image_to_base64(&file_path).unwrap();

    let response = api::send_image_to_open_ai(
        base64_image,
        OPEN_API_KEY.to_string(),
        OPEN_API_PROMPT.to_string(),
    )
    .await;

    // println!("response: {:?}", response);
    match response {
        Ok(response) => {
            println!("response: {:?}", response);
            Ok(response)
        },
        Err(error) => {
            println!("error: {:?}", error);
            Err("Error while converting image to code".to_string())
        }
    }
}
