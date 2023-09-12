use rusty_tesseract::TessError;
use thiserror::Error;
use std::{fmt::{self, Display}, error::Error};

#[derive(Error, Debug)]
pub struct TessErrWrapper {
    pub error: TessError,
}

#[derive(Error, Debug)]
pub enum OCRError {
    #[error("Tesseract failed to extract text from the image")]
    OCRTessErr(#[from] TessErrWrapper),

    #[error("There was a problem saving the image to your disk")]
    OCRioErr(#[from] std::io::Error ),
}

pub fn err_to_string(e: OCRError) -> String {
    match e.source() {
        Some(source) => {
            format!("Error: {}\n    Caused by: {}", e, source)
        },
        None => format!("Error: {}\n", e)
    }
}

impl fmt::Display for TessErrWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            tess_err => 
                write!(f, "{}", tess_err.error.to_string())
        }
    }
}

impl From<TessError> for OCRError {
    fn from(err: TessError) -> OCRError {
        OCRError::OCRTessErr(TessErrWrapper { error: err })
    }
}

pub type Result<T> = std::result::Result<T, OCRError>;