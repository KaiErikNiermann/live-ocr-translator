use image::io::Reader as ImageReader;
use rusty_tesseract::{Image, Args};

pub fn get_image(path: &str) -> Image {
    let img = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap();

    return Image::from_dynamic_image(&img).unwrap();
}

pub fn text_from_image(img: &Image, args: &Args) -> String {
    rusty_tesseract::image_to_string(img, args)
        .unwrap()
} 