// 2 つの共有借用を同時に読む練習です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `longer_len`
// - 引数: `a: &str`, `b: &str`
// - 戻り値: `usize`
//
// 条件:
// - 長い方の長さを返す
// - 長さが同じならどちらでもよい
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn longer_len(a: &str, b: &str) -> usize {
    let len_a = a.len();
    let len_b = b.len();

    if len_a > len_b { len_a } else { len_b }
}

#[cfg(test)]
mod tests {
    use super::longer_len;

    #[test]
    fn returns_length_of_longer_input() {
        assert_eq!(longer_len("abc", "abcdef"), 6);
    }

    #[test]
    fn returns_length_when_equal() {
        assert_eq!(longer_len("cat", "dog"), 3);
    }
}
