use reqwest;
use serde::Deserialize;
use serde_json::{json, Value};
use std::error::Error;

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

    let body = json!({
        "model": "gpt-4-vision-preview",
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": prompt
                    },
                    {
                        "type": "image_url",
                        "image_url":  {
                            "url": format!("data:image/png;base64,{}", base64_image),
                        }
                    }
                ]
            }
        ],
        "max_tokens": 3000
    });

    println!("request_body: {:?}", json!(body));

    let res = client
        .post(url)
        .body(body.to_string())
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
