pub mod image;
pub mod text;
pub mod win_sc;

pub fn get_tesseract_supported() -> Vec<String> {
    rusty_tesseract::get_tesseract_langs().unwrap()
}

pub fn run_ocr(path: &str, lang: &str) -> String {
    match image::get_image(path) {
        Ok(img) => {
            return text::clean_text(&image::text_from_image(
                &img,
                &(rusty_tesseract::Args {
                    lang: String::from(lang),
                    ..Default::default()
                }),
            ));
        },
        Err(err_msg) => {
            return err_msg;
        }
    }; 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ocr_example() {

    }

    #[test]
    fn get_tesseract_supported() {

    }

    #[test]
    fn advanced_ocr_example() {
        
    }
}
