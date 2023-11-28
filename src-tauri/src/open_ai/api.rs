use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

#[derive(Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize)]
struct ImageMessage {
    #[serde(rename = "type")]
    message_type: String,
    text: Option<String>,
    image_url: Option<ImageUrl>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: Vec<ImageMessage>,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct ResponseChoice {
    message: Value,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    error: Option<Value>,
    choices: Option<Vec<ResponseChoice>>,
}

pub async fn send_image_to_open_ai(
    base64_image: String,
    open_api_key: String,
    prompt: String,
) -> Result<String, Box<dyn Error>> {
    let url = "https://api.openai.com/v1/chat/completions";
    let client = reqwest::Client::new();

    //   curl https://api.openai.com/v1/chat/completions \
    // -H "Content-Type: application/json" \
    // -H "Authorization: Bearer $OPENAI_API_KEY" \
    // -d '{
    //   "model": "gpt-4-vision-preview",
    //   "messages": [
    //     {
    //       "role": "user",
    //       "content": [
    //         {
    //           "type": "text",
    //           "text": "What’s in this image?"
    //         },
    //         {
    //           "type": "image_url",
    //           "image_url": {
    //             "url": "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg"
    //           }
    //         }
    //       ]
    //     }
    //   ],
    //   "max_tokens": 300
    // }'
    let image_url = ImageUrl {
        url: format!("data:image/png;base64,{}", base64_image),
    };
    let request_body = RequestBody {
        model: "gpt-4-vision-preview".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: vec![
                ImageMessage {
                    message_type: "text".to_string(),
                    text: Some(prompt.to_string()),
                    image_url: None,
                },
                ImageMessage {
                    message_type: "image_url".to_string(),
                    text: None,
                    image_url: Some(image_url),
                },
            ],
        }],
        max_tokens: 3000,
    };

    println!("request_body: {:?}", json!(request_body));

    let res = client
        .post(url)
        .json(&request_body)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", open_api_key))
        .send()
        .await?;

    if res.status().is_success() {
        let response_body: OpenAIResponse = res.json().await?;
        println!("response_body: {:?}", response_body);
        if let Some(error) = response_body.error {
            Err(format!("OpenAI Error: {:?}", error).into())
        } else if let Some(choices) = response_body.choices {
            let first_choice = &choices[0];
            let content = first_choice.message["content"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            Ok(content)
        } else {
            Err("No response from OpenAI".into())
        }
    } else {
        println!("Request failed with status: {:?}", res);
        Err(format!("Request failed with status: {}", res.status()).into())
    }
}
