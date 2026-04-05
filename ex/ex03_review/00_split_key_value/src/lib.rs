/// `key=value` 形式の文字列を、借用のまま2つの `&str` に分ける課題です。
///
/// 条件:
/// - 最初の `=` だけで分割する
/// - キーか値が空なら `None`
/// - 新しい `String` は作らない
#[allow(unused_variables)]
pub fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let sep = "=";

    let first_sep_pos = line.find(sep)?;
    if first_sep_pos == 0 || first_sep_pos + sep.len() == line.len() {
        return None;
    }
    Some((&line[..first_sep_pos], &line[first_sep_pos + sep.len()..]))
}

pub fn split_key_value_2(line: &str) -> Option<(&str, &str)> {
    let first_sep_pos = line.find('=')?;
    let (key, value_with_sep) = line.split_at(first_sep_pos);
    let value = {
        let mut it = value_with_sep.chars();
        it.next();
        it.as_str()
    };
    if key.is_empty() || value.is_empty() {
        return None;
    }
    Some((key, value))
}

pub fn split_key_value_3(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once('=')?;
    if key.is_empty() || value.is_empty() {
        return None;
    }
    Some((key, value))
}

#[cfg(test)]
mod tests {
    use super::split_key_value;

    #[test]
    fn splits_simple_pair() {
        assert_eq!(split_key_value("name=Alice"), Some(("name", "Alice")));
    }

    #[test]
    fn uses_first_equal_only() {
        assert_eq!(split_key_value("path=a=b=c"), Some(("path", "a=b=c")));
    }

    #[test]
    fn returns_none_when_separator_is_missing() {
        assert_eq!(split_key_value("name"), None);
    }

    #[test]
    fn returns_none_when_key_or_value_is_empty() {
        assert_eq!(split_key_value("=Alice"), None);
        assert_eq!(split_key_value("name="), None);
    }
}
