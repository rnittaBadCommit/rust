// `String` を値で受け取り、所有権ごと扱う課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `take_and_report`
// - 引数: `s: String`
// - 戻り値: `(String, usize)`
//
// 条件:
// - 受け取った文字列そのものと長さを返す
// - 戻り値の型は `(String, usize)`
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn take_and_report(s: String) -> (String, usize) {
    let len_size = s.chars().count();
    (s, len_size)
}

#[cfg(test)]
mod tests {
    use super::take_and_report;

    #[test]
    fn returns_original_string_and_length() {
        let (s, len) = take_and_report(String::from("rust"));
        assert_eq!(s, "rust");
        assert_eq!(len, 4);
    }

    #[test]
    fn handles_empty_string() {
        let (s, len) = take_and_report(String::new());
        assert_eq!(s, "");
        assert_eq!(len, 0);
    }
}
