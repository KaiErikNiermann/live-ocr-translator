use reqwest::{Response, StatusCode};
use std::{
    error::Error,
    fmt::{self, Display},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranslatorError {
    #[error("Authorization failed, API key is likely wrong")]
    AuthorizationErr,

    #[error("An error occured when communicating with the DeepL server\n    Error: {0}")]
    ServerErr(String),

    #[error("An error occured when deserializing the server response")]
    DeserializationErr,

    #[error("The requested resource was not found")]
    NotFoundErr,
}

pub type Result<T> = std::result::Result<T, TranslatorError>;
