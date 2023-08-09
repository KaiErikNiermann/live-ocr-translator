pub fn translate() {
    println!("translator lib");
}

pub fn translate_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        translate();
    }

    #[test]
    fn test_translate_add() {
        assert_eq!(translate_add(1, 2), 3);
    }
}