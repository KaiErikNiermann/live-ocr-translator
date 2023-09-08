use reqwest::{Client, Request, Response};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct TransResponseBody {
    detected_source_language: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TranslationResponse {
    translations: Vec<TransResponseBody>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    language: String,
    name: String,
    supports_formality: bool
}

pub async fn translate_text(text: &str) -> Result<String, reqwest::Error> {
    let auth_key = env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY key is not set, set it in a .env file in the root dir of the project");
    let client = Client::new();
    let url = "https://api-free.deepl.com/v2/translate";

    // Create the JSON data payload.
    let json_data = format!(
        r#"{{
            "text": ["{}"],
            "target_lang": "DE"
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

    match response.error_for_status() {
        Ok(res) => {
            let string_res = res.text().await.unwrap();
            let api_res: TranslationResponse = serde_json::from_str(&string_res).unwrap();

            println!("{:?}", api_res.translations[0].text);
            println!("Ok!");
            Ok(String::from(api_res.translations[0].text.clone()))
        }
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
    }
}

pub async fn get_supported() -> Result<Vec<Language>, reqwest::Error> {
    let auth_key = env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY key is not set, set it in a .env file in the root dir of the project");
    let client = Client::new();
    let url = "https://api-free.deepl.com/v2/languages?type=target";

    let response = client
        .get(url)
        .header("Authorization", format!("DeepL-Auth-Key {}", auth_key))
        .send()
        .await
        .unwrap();

    match response.error_for_status() {
        Ok(res) => {
            let string_res = res.text().await.unwrap();
            let api_res: Vec<Language> = serde_json::from_str(&string_res).unwrap();

            Ok(api_res)
        },
        Err(e) => Err(e)
    }
}