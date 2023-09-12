use std::{
    error::Error,
    fmt::{self, Display},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowsCaptureError {
    #[error("Failed to find window handle, is the translator window open?")]
    WindowNotFoundErr(#[source] windows::core::Error),

    #[error("Failed to get the dimensions of the window, cant capture region")]
    DimensionNotFoundErr(#[source] windows::core::Error),

    #[error("Failed to create image from frame data")]
    ImageGenFailedErr(#[source] windows::core::Error),
}

pub fn err_to_string(e: WindowsCaptureError) -> String {
    match e.source() {
        Some(source) => {
            format!("Error: {}\n    Caused by: {}", e, source)
        }
        None => format!("Error: {}\n", e),
    }
}
