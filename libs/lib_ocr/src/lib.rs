pub mod image;
pub mod text;
pub mod win_sc;

pub fn run_ocr(path: &str, lang: &str) -> String {
    let img = image::get_image(path);
    return text::clean_text(&image::text_from_image(&img, &(rusty_tesseract::Args {
        lang: String::from(lang),
        ..Default::default()
    })));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }

    #[test]
    fn test_ocr_add() {
    }
}