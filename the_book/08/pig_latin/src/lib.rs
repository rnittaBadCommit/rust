fn starts_with_vowel(ch: char) -> bool {
    matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn split_trailing_punctuation(word: &str) -> (&str, &str) {
    let Some((punctuation_start, last_body_char)) = word
        .char_indices()
        .rev()
        .find(|(_, ch)| ch.is_alphanumeric())
    else {
        return ("", word);
    };

    let word_end = punctuation_start + last_body_char.len_utf8();
    (&word[..word_end], &word[word_end..])
}

pub fn pig_latin_word(word: &str) -> String {
    let (body, punctuation) = split_trailing_punctuation(word);
    let Some(first) = body.chars().next() else {
        return punctuation.to_string();
    };

    if starts_with_vowel(first) {
        return format!("{body}-hay{punctuation}");
    }

    let first_len = first.len_utf8();
    let rest = &body[first_len..];

    format!("{rest}-{first}ay{punctuation}")
}

pub fn pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(pig_latin_word)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{pig_latin, pig_latin_word};

    #[test]
    fn moves_first_consonant_to_end() {
        assert_eq!(pig_latin_word("first"), "irst-fay");
        assert_eq!(pig_latin_word("rust"), "ust-ray");
    }

    #[test]
    fn adds_hay_to_vowel_words() {
        assert_eq!(pig_latin_word("apple"), "apple-hay");
        assert_eq!(pig_latin_word("Orange"), "Orange-hay");
    }

    #[test]
    fn handles_utf8_without_breaking_char_boundaries() {
        assert_eq!(pig_latin_word("éclair"), "clair-éay");
        assert_eq!(pig_latin_word("んrust"), "rust-んay");
    }

    #[test]
    fn converts_each_word_in_text() {
        assert_eq!(pig_latin("first apple rust"), "irst-fay apple-hay ust-ray");
    }

    #[test]
    fn keeps_trailing_punctuation_at_the_end() {
        assert_eq!(pig_latin_word("first."), "irst-fay.");
        assert_eq!(pig_latin_word("apple!"), "apple-hay!");
        assert_eq!(pig_latin("first apple."), "irst-fay apple-hay.");
    }

    #[test]
    fn handles_multiple_trailing_punctuation_marks() {
        assert_eq!(pig_latin_word("rust?!"), "ust-ray?!");
        assert_eq!(pig_latin_word("..."), "...");
    }

    #[test]
    fn empty_word_stays_empty() {
        assert_eq!(pig_latin_word(""), "");
    }
}
