use reqwest::{Client, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct translations {
    detected_source_language: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    translations: Vec<translations>,
}

pub async fn translate_text(text: &str, auth_key: &str) -> Result<String, String> {
    let client = Client::new();
    let url = "https://api-free.deepl.com/v2/translate";

    // Create the JSON data payload.
    let json_data = format!(
        r#"{{
            "text": ["{}"],
            "target_lang": "EN"
        }}"#,
        text
    );

    // Build the request with headers.
    let response = client
        .post(url)
        .header("Authorization", format!("DeepL-Auth-Key {}", auth_key))
        .header("Content-Type", "application/json")
        .body(json_data)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let string_res = response.text().await.unwrap();
            let api_res: APIResponse = serde_json::from_str(&string_res).unwrap();

            println!("{:?}", api_res.translations[0].text);
            println!("Ok!");
            Ok(String::from(api_res.translations[0].text.clone()))
        }
        reqwest::StatusCode::BAD_REQUEST => {
            println!("Bad request");
            Err(String::from("Error"))
        }
        _ => {
            panic!("Something unexpected");
        }
    }
}
