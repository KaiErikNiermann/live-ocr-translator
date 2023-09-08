use reqwest::{Client};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct translations {
    detected_source_language: String, 
    text: String
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    translations: Vec<translations>
}

pub async fn translate_text(text: &str, auth_key: &str) {
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

    // Check if the request was successful (status code 200).
    match response.status() {
        reqwest::StatusCode::OK => {
            
            let string_res = response.text().await.unwrap();

            let api_res: APIResponse = serde_json::from_str(&string_res).unwrap();

            println!("{:?}", api_res.translations[0].text);
            println!("Ok!")
        },
        reqwest::StatusCode::BAD_REQUEST => {
            println!("Bad request")
        },
        _ => {
            panic!("Something unexpected")
        }
    }
}
