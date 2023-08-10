pub fn clean_text(text: &str) -> String {
    return text
    .replace("\n", "")
    .split_whitespace()
    .collect::<Vec<_>>()
    .iter()
    .enumerate()
    .map(|(i, word)| {
        if i % 10 == 0 {
            format!("{}\n", word)
        } else {
            format!("{} ", word)
        }
    })
    .collect::<Vec<_>>()
    .join("");
}