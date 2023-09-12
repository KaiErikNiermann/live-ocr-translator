use image::io::Reader as ImageReader;
use rusty_tesseract::{Args, Image};
use std::env;

use crate::errors;
use crate::errors::*;

pub fn get_image(path: &str) -> errors::Result<Image> {
    let path = format!("{}/{}", env::current_dir().unwrap().to_str().unwrap(), path);

    match ImageReader::open(path) {
        Ok(res) => {
            let dynimg = res.decode().unwrap();

            match Image::from_dynamic_image(&dynimg) {
                Ok(image) => Ok(image),
                Err(e) => Err(OCRError::OCRTessErr(TessErrWrapper { error: e }))
            }
        }
        Err(err) => {
            Err(OCRError::OCRioErr(err))
        }
    }
}

pub fn text_from_image(img: &Image, args: &Args) -> errors::Result<String> {
    match rusty_tesseract::image_to_string(img, args) {
        Ok(res) => Ok(res),
        Err(e) => Err(OCRError::OCRTessErr(TessErrWrapper { error: e })),
    }
}
