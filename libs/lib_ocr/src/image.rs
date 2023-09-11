use image::io::Reader as ImageReader;
use rusty_tesseract::{Args, Image};
use std::env;

pub fn get_image(path: &str) -> Result<Image, String> {
    if let Ok(current_dir) = env::current_dir() {
        if let Some(dir_str) = current_dir.to_str() {
            println!("Current Working Directory: {}", dir_str);
        } else {
            println!("Unable to convert current directory to string.");
        }
    } else {
        println!("Unable to retrieve current directory.");
    }

    let path = format!("{}/{}", env::current_dir().unwrap().to_str().unwrap(), path);

    match ImageReader::open(path) {
        Ok(res) => {
            let img = res.decode().unwrap();
            return Ok(Image::from_dynamic_image(&img).unwrap());
        }
        Err(err) => {
            return Err(format!("{:?}", err));
        }
    };
}

pub fn text_from_image(img: &Image, args: &Args) -> String {
    return match rusty_tesseract::image_to_string(img, args) {
        Ok(res) => res,
        Err(_) => String::from("OCR failed"),
    };
}
