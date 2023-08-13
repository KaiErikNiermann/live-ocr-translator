use image::io::Reader as ImageReader;
use rusty_tesseract::{Image, Args};
use std::env;
use std::fs;

pub fn get_image(path: &str) -> Image {
    // print cwd 
    if let Ok(current_dir) = env::current_dir() {
        if let Some(dir_str) = current_dir.to_str() {
            println!("Current Working Directory: {}", dir_str);
        } else {
            println!("Unable to convert current directory to string.");
        }
    } else {
        println!("Unable to retrieve current directory.");
    }

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    // concat cwd with path
    let path = format!("{}/{}", env::current_dir().unwrap().to_str().unwrap(), path);
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