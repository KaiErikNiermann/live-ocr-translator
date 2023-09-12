use rusty_tesseract::TessError;
use rusty_tesseract::image::DynamicImage;
use thiserror::Error;
use std::error;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt::{self, Display};

#[derive(Error, Debug)]
pub struct TessErrWrapper {
    pub error: TessError,
}

#[derive(Error, Debug)]
pub enum OCRError {
    #[error("Tesseract failed to extract text from the image")]
    OCRTessErr(#[from] TessErrWrapper),

    #[error("There was a problem saving the image to your disk")]
    OCRioErr(#[from] std::io::error)
}

impl Display::fmt for TessErrWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TessErrWrapper => write(f, "{}", TessErrWrapper.error.to_string())
        }
    }
}

impl From<TessError> for OCRError {
    fn from(err: TessError) -> OCRError {
        OCRError::OCRTessErr(TessErrWrapper { error: err })
    }

    fn from(err: std::io::Error) {
        OCRError::OCRioErr(err)
    }
}

pub type Result<T> = std::result::Result<T, OCRError>;