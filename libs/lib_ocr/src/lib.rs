pub fn ocr() {
    println!("ocr lib");
}

pub fn ocr_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        ocr();
    }

    #[test]
    fn test_ocr_add() {
        assert_eq!(ocr_add(1, 2), 3);
    }
}