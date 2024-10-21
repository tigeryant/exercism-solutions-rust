use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut current_word = String::new();
    let mut word_map: HashMap<String, u32> = HashMap::new();

    let lowercase_words = words.to_ascii_lowercase();
    let chars: Vec<char> = lowercase_words.chars().collect();
    let mut char_iter = chars.iter().peekable();

    while let Some(&char) = char_iter.next() {
        match char {
            c if c.is_alphanumeric() => current_word.push(c),
            '\'' => {
                if current_word.is_empty() { // open quote
                    continue;
                } 
                match char_iter.peek() {
                    None => { // close quote
                        continue;
                    },
                    Some(&next_char) if next_char.is_whitespace() => { // close quote
                        continue;
                    },
                    Some(&next_char) if next_char.is_alphabetic() => { // contraction
                        current_word.push('\'');
                    },
                    Some(_) => { // default
                        continue;
                    }
                }
            },
            c if c.is_ascii_punctuation() || c.is_whitespace() => { // 
                if !current_word.is_empty() {
                    let count = word_map.entry(current_word.clone()).or_insert(0);
                    *count += 1;
                    current_word.clear();
                }
            },
            _ => (),
        };
    }

    // if current_word is not empty after the chars iter is exhausted, add it to the word_map
    if !current_word.is_empty() {
        *word_map.entry(current_word).or_insert(0) += 1;
    }

    word_map
}
