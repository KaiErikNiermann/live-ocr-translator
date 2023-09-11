use reqwest::{self, dns::Resolving, Client, Error, Method, Request, Response};
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

    fn request_handler(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let base_url = "https://api-free.deepl.com/v2";
        let url = format!("{}{}", base_url, endpoint);
        let request = client
            .request(method.clone(), url)
            .header("Authorization", format!("DeepL-Auth-Key {}", self.auth_key));

        let response = match params {
            Some(params) => match method {
                Method::GET => request.query(params).send(),
                Method::PATCH | Method::POST | Method::PUT => request.form(params).send(),
                _ => unreachable!("Only GET, PATCH, POST and PUT are supported with params."),
            },
            None => request.send(),
        };

        let res = match response {
            Ok(response) if response.status().is_success() => {
                println!("Successful API request");
                response
            }
            Ok(response) if response.status() == reqwest::StatusCode::UNAUTHORIZED => {
                println!("Unauthorized request");
                response.error_for_status()?
            }
            Ok(response) if response.status() == reqwest::StatusCode::FORBIDDEN => {
                response.error_for_status()?
            }
            Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                response.error_for_status()?
            }
            Ok(response) => response,
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        Ok(res)
    }

    pub fn translate_text(&self, text: &str, target_lang: &str) -> Result<String, reqwest::Error> {
        let query = vec![("target_lang", target_lang), ("text", text)];

        let response = self.request_handler(Method::POST, "/translate", Some(&query))?;

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
        let response = self.request_handler(Method::GET, "/languages?type=target", None)?;

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

#[cfg(test)]
mod tests {
    use crate::DeepL;
    use dotenv::dotenv;
    use std::env;

    fn translator() -> DeepL {
        dotenv().ok();
        let auth_key = env::var("DEEPL_API_KEY").expect(
            "DEEPL_API_KEY key is not set, set it in a .env file in the root dir of the project",
        );
        DeepL::new(auth_key)
    }

    #[test]
    fn basic_translation() {
        let text: &str = "This is a basic sentence I want to translate.";
        match translator().translate_text(text, "DE") {
            Ok(translated_text) => {
                assert_eq!(
                    translated_text,
                    "Dies ist ein einfacher Satz, den ich übersetzen möchte."
                )
            }
            Err(e) => panic!("Failed to translated text: {:?}", e),
        }
    }

    #[test]
    fn get_supported_languages() {
        match translator().get_supported() {
            Ok(supported_langs) => {
                assert!(!supported_langs.is_empty())
            }
            Err(e) => panic!("Failed to get supported langs: {:?}", e),
        }
    }
}
