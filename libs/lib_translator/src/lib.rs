use reqwest::{Client, Request, Response};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct TranslationResponseBody {
    detected_source_language: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TranslationResponse {
    translations: Vec<TranslationResponseBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Language {
    pub language: String,
    pub name: String,
    pub supports_formality: bool,
}

#[derive(Clone)]
pub struct DeepL {
    pub auth_key: String,
}

impl DeepL {
    pub fn new(auth_key: String) -> DeepL {
        DeepL { auth_key: auth_key }
    }

    pub fn set(&mut self, new_auth_key: String) {
        self.auth_key = new_auth_key;
    }

    pub fn translate_text(
        &self,
        text: &str,
        target_lang: &str,
    ) -> Result<String, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = "https://api-free.deepl.com/v2/translate";

        // Create the JSON data payload.
        let json_data = format!(
            r#"{{
                "text": ["{}"],
                "target_lang": "{}"
            }}"#,
            text, target_lang
        );

        // Build the request with headers.
        let response = client
            .post(url)
            .header("Authorization", format!("DeepL-Auth-Key {}", self.auth_key))
            .header("Content-Type", "application/json")
            .body(json_data)
            .send()
            .unwrap();

        match response.error_for_status() {
            Ok(res) => {
                let string_res = res.text().unwrap();
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

    pub fn get_supported(&self) -> Result<Vec<Language>, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = "https://api-free.deepl.com/v2/languages?type=target";

        let response = client
            .get(url)
            .header("Authorization", format!("DeepL-Auth-Key {}", self.auth_key))
            .send()
            .unwrap();

        match response.error_for_status() {
            Ok(res) => {
                let string_res = res.text().unwrap();
                let api_res: Vec<Language> = serde_json::from_str(&string_res).unwrap();

                Ok(api_res)
            }
            Err(e) => Err(e),
        }
    }
}
