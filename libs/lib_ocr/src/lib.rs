use ::image::DynamicImage;
use errors::TessErrWrapper;
use rusty_tesseract::{Args, Image};

pub mod image;
pub mod text;

#[cfg(target_os = "windows")]
pub mod win_sc;

pub mod errors;

pub fn get_tesseract_supported() -> Vec<String> {
    rusty_tesseract::get_tesseract_langs().unwrap()
}

pub fn run_ocr(path: &str, lang: &str) -> errors::Result<String> {
    match image::get_image(path) {
        Ok(img) => ocr_img(&img, lang),
        Err(err_msg) => Err(err_msg),
    }
}

fn ocr_img(img: &Image, lang: &str) -> errors::Result<String> {
    let raw_text = image::text_from_image(
        &img,
        &(rusty_tesseract::Args {
            lang: String::from(lang),
            ..Default::default()
        }),
    )?;

    return Ok(text::clean_text(&raw_text));
}

pub fn run_ocr_img(img: &DynamicImage, lang: &str) -> errors::Result<String> {
    match Image::from_dynamic_image(img) {
        Ok(image) => ocr_img(&image, lang),
        Err(e) => Err(errors::OCRError::OCRTessErr(TessErrWrapper { error: e })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ocr_example() {}

    #[test]
    fn get_tesseract_supported() {}

    #[test]
    fn advanced_ocr_example() {}
}
