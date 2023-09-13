use image::DynamicImage;
use image::io::Reader as ImageReader;
use rusty_tesseract::{Args, Image};
use std::env;

use crate::errors;
use crate::errors::*;

pub fn get_image(path: &str) -> errors::Result<Image> {
    let path = format!("{}/{}", env::current_dir().unwrap().to_str().unwrap(), path);

    match ImageReader::open(path) {
        Ok(res) => {
            let dynamic_image = res.decode().unwrap();
            image_from_dynamic(&dynamic_image)
        }
        Err(err) => Err(OCRError::OCRioErr(err)),
    }
}

pub fn image_from_dynamic(dynamic_image: &DynamicImage) -> errors::Result<Image> {
    match Image::from_dynamic_image(dynamic_image) {
        Ok(image) => Ok(image),
        Err(e) => Err(OCRError::OCRTessErr(TessErrWrapper { error: e }))
    }
}

pub fn text_from_image(img: &Image, args: &Args) -> errors::Result<String> {
    match rusty_tesseract::image_to_string(img, args) {
        Ok(res) => Ok(res),
        Err(e) => Err(OCRError::OCRTessErr(TessErrWrapper { error: e })),
    }
}
