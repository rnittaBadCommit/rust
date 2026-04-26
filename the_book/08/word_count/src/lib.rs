use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut word_count: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *(word_count.entry(word.to_string()).or_insert(0)) += 1;
    }
    word_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words_with_entry() {
        let counts = word_count("hello rust hello");

        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("rust"), Some(&1));
        assert_eq!(counts.get("missing"), None);
    }
}
