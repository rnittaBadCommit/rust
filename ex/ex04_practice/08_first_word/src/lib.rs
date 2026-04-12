// 文字列から最初の単語を、借用のまま返す課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `first_word`
// - 引数: `s: &str`
// - 戻り値: `Option<&str>`
//
// 条件:
// - 空白で区切った最初の単語を返す
// - 見つからなければ `None`
// - 新しい `String` は作らない
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn first_word(s: &str) -> Option<&str> {
    let mut it = s.char_indices();

    while let Some((i, c)) = it.next() {
        if c.is_whitespace() == false {
            let word_head = i;
            while let Some((i, c)) = it.next() {
                if c.is_whitespace() {
                    return Some(&s[word_head..i]);
                }
            }
            return Some(&s[word_head..]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn returns_first_word() {
        assert_eq!(first_word("alpha beta gamma"), Some("alpha"));
    }

    #[test]
    fn skips_leading_spaces() {
        assert_eq!(first_word("   hello rust"), Some("hello"));
    }

    #[test]
    fn returns_none_when_no_word_exists() {
        assert_eq!(first_word("   \t  "), None);
    }

    #[test]
    fn returns_none_for_empty_string() {
        assert_eq!(first_word(""), None);
    }

    #[test]
    fn handles_unicode_whitespace() {
        assert_eq!(first_word("\u{3000}hello rust"), Some("hello"));
    }
}
