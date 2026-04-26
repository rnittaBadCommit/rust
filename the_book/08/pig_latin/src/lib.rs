pub fn pig_latin_word(word: &str) -> String {
    let _ = word;
    todo!("先頭が母音なら -hay、子音なら先頭文字を末尾へ移して -ay を付ける")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vowel_word_gets_hay() {
        assert_eq!(pig_latin_word("apple"), "apple-hay");
    }

    #[test]
    fn consonant_word_moves_first_char() {
        assert_eq!(pig_latin_word("first"), "irst-fay");
    }

    #[test]
    fn handles_utf8_char_boundary() {
        assert_eq!(pig_latin_word("猫cat"), "cat-猫ay");
    }
}
