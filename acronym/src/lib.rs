pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let words = phrase.split([' ', '-']);
    for word in words {
        let upper_chars = word.chars().filter(|c| c.is_uppercase()).collect::<Vec<char>>();
        let has_lowercase = word.chars().any(|c| c.is_lowercase());
        if !upper_chars.is_empty() && has_lowercase {
            for c in upper_chars {
                result.push(c);
            }
        } else if let Some(char) = word.chars().next() {
            result.push(char.to_ascii_uppercase())
        }
    }
    result
}
