/// Cの `strlen` を思い出しながら、Rustの `&str` と `len()` に慣れる課題です。
///
/// 注意:
/// - Rustの `&str` はUTF-8
/// - `len()` は文字数ではなくバイト数
#[allow(unused_variables)]
pub fn my_strlen(s: &str) -> usize {
    // todo!("`&str` を受け取り、バイト数を返してください");
    let mut ret: usize = 0;

    for c in s {
        ret += 1;
    }
    ret
}

pub fn my_strlen_2(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::my_strlen;

    #[test]
    fn empty_string_has_len_zero() {
        assert_eq!(my_strlen(""), 0);
    }

    #[test]
    fn ascii_string_len_matches_c_style_expectation() {
        assert_eq!(my_strlen("hello"), 5);
    }

    #[test]
    fn utf8_len_is_byte_count_not_character_count() {
        assert_eq!(my_strlen("é"), 2);
    }
}
