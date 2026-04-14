// 文字列の長さを順に集める課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `collect_lengths`
// - 引数: `words: &[String]`
// - 戻り値: `Vec<usize>`
//
// 条件:
// - 各文字列の長さを順に入れた `Vec` を返す
// - `words` の所有権は奪わない
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

#[cfg(test)]
mod tests {
    use super::collect_lengths;

    #[test]
    fn collects_lengths_in_order() {
        let words = vec![
            String::from("rust"),
            String::from(""),
            String::from("borrow"),
        ];
        assert_eq!(collect_lengths(&words), vec![4, 0, 6]);
    }

    #[test]
    fn does_not_consume_input_strings() {
        let words = vec![String::from("alpha"), String::from("beta")];
        let lengths = collect_lengths(&words);
        assert_eq!(lengths, vec![5, 4]);
        assert_eq!(words[0], "alpha");
        assert_eq!(words[1], "beta");
    }
}
