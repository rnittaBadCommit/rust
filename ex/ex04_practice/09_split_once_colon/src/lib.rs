// `key:value` 形式の文字列を、借用のまま 2 つに分ける課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `split_once_colon`
// - 引数: `s: &str`
// - 戻り値: `Option<(&str, &str)>`
//
// 条件:
// - 最初の `:` だけで分割する
// - 前半か後半が空なら `None`
// - 新しい `String` は作らない
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn split_once_colon(s: &str) -> Option<(&str, &str)> {
    let mut it = s.char_indices();
    let mut first_word: Option<&str> = None;
    while let Some((i, c)) = it.next() {
        if c == ':' {
            first_word = Some(&s[..i]);
            break;
        }
    }
    if first_word == None || first_word.unwrap().is_empty() {
        return None;
    }
    match it.next() {
        Some((i, _)) => return Some((first_word.unwrap(), &s[i..])),
        None => return None,
    }
}

pub fn split_onece_colon_1_2(s: &str) -> Option<(&str, &str)> {
    let mut it = s.char_indices();
    while let Some((i, c)) = it.next() {
        if c == ':' {
            let left_word = &s[..i];
            let right_word = &s[i + ':'.len_utf8()..];
            if left_word.is_empty() || right_word.is_empty() {
                return None;
            } else {
                return Some((left_word, right_word));
            }
        }
    }
    None
}

pub fn split_once_colon2(s: &str) -> Option<(&str, &str)> {
    let (first_word, second_word) = s.split_once(':')?;
    if first_word.is_empty() || second_word.is_empty() {
        return None;
    }
    Some((first_word, second_word))
}

pub fn split_once_colon3fifi(s: &str) -> Option<(&str, &str)> {
    s.split_once(':')
        .filter(|(left, right)| !left.is_empty() && !right.is_empty())
}

#[cfg(test)]
mod tests {
    use super::split_once_colon;

    #[test]
    fn splits_simple_pair() {
        assert_eq!(split_once_colon("host:8080"), Some(("host", "8080")));
    }

    #[test]
    fn uses_first_colon_only() {
        assert_eq!(split_once_colon("a:b:c"), Some(("a", "b:c")));
    }

    #[test]
    fn rejects_missing_or_empty_parts() {
        assert_eq!(split_once_colon("abc"), None);
        assert_eq!(split_once_colon(":8080"), None);
        assert_eq!(split_once_colon("host:"), None);
    }
}
