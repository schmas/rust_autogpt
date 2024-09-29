use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

// Call Large Language Mode (i.e. GPT-4o)
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extract API Key Information
    let api_key: String =
        env::var("OPEN_API_KEY").expect("OPEN_API_KEY not found in environment variables");
    let api_org: String =
        env::var("OPEN_API_ORG").expect("OPEN_API_ORG not found in environment variables");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers: HeaderMap = HeaderMap::new();

    // Create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Create OpenAPI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Create the client
    let client = Client::builder().default_headers(headers).build().unwrap();

    // Create chat completion
    let chat_completion = ChatCompletion {
        model: "gpt-4o".to_string(),
        messages,
        temperature: 0.1,
    };

    // Troubleshooting
    let res_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn tests_call_to_openai() {
        let messages = vec![Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response.".to_string(),
        }];
        call_gpt(messages).await;
    }
}
