pub fn clean_text(text: &str) -> String {
    text.clone()
        .replace("\n", " ")
        .replace("\t", " ")
}