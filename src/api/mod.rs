use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};

use crate::error::Error;

pub async fn query_chat_gpt (token: &str, query: &str) -> Result<Value, Error> {
    let api = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{
            "role": "user", 
            "content": query.to_string()
        }]
    });


    let res = api
        .post(url)
        .headers(headers(token))
        .body(body.to_string())
        .send()
        .await?;


    let json: Value = res.json().await?;

    let message = &json["choices"][0]["message"]["content"];

    Ok(message.clone())
}

fn headers(token: &str) -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().expect("could not format authorization header"));
    headers.insert(CONTENT_TYPE, "application/json".parse().expect("could not format authorization header"));

    headers
}
