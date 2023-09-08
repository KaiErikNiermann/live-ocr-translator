pub fn clean_text(text: &str) -> String {
    return text
        .replace("\n", " ")
        .replace("\"", "\\\"")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
}
