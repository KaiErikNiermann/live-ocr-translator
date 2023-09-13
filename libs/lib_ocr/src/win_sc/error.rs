use std::{
    error::Error,
    fmt::{self, Display},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowsCaptureError {
    #[error("Failed to find window handle, is the translator window open?")]
    WindowNotFoundErr,

    #[error("Failed to get the dimensions of the window, cant capture region")]
    DimensionNotFoundErr(#[source] windows::core::Error),

    #[error("Failed to create image from frame data")]
    ImageGenFailedErr(#[source] windows::core::Error),

    #[error("Failed to create image from path")]
    ImageSaveFailedErr(#[source] std::io::Error),
}

pub fn err_to_string(e: &WindowsCaptureError) -> String {
    match e.source() {
        Some(source) => {
            format!("Error: {}\n    Caused by: {}", e, source)
        }
        None => format!("Error: {}\n", e),
    }
}

impl From<windows::core::Error> for WindowsCaptureError {
    fn from(error: windows::core::Error) -> Self {
        WindowsCaptureError::ImageGenFailedErr(error)
    }
}

pub type Result<T> = std::result::Result<T, WindowsCaptureError>;