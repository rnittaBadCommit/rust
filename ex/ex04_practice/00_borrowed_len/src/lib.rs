// 文字列を読むだけの関数は、まず `&str` で受ける練習です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `borrowed_len`
// - 引数: `s: &str`
// - 戻り値: `usize`
//
// 条件:
// - 長さを返すだけにする
// - 新しい `String` は作らない
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn borrowed_len(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::borrowed_len;

    #[test]
    fn returns_length_of_ascii_string() {
        assert_eq!(borrowed_len("hello"), 5);
    }

    #[test]
    fn returns_zero_for_empty_string() {
        assert_eq!(borrowed_len(""), 0);
    }
}
