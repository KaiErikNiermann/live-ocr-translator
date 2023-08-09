pub fn gui() {
    println!("gui lib");
}

pub fn gui_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        gui();
    }

    #[test]
    fn test_gui_add() {
        assert_eq!(gui_add(1, 2), 3);
    }
}