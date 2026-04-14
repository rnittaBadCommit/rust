// 空文字列を飛ばして `,` で連結する課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `join_nonempty`
// - 引数: `words: &[&str]`
// - 戻り値: `String`
//
// 条件:
// - 空文字列は飛ばす
// - 残った要素だけを `,` でつなぐ
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

#[cfg(test)]
mod tests {
    use super::join_nonempty;

    #[test]
    fn joins_only_nonempty_words() {
        assert_eq!(
            join_nonempty(&["alpha", "", "beta", "gamma"]),
            "alpha,beta,gamma"
        );
    }

    #[test]
    fn returns_empty_string_when_all_words_are_empty() {
        assert_eq!(join_nonempty(&["", "", ""]), "");
    }

    #[test]
    fn avoids_extra_separator_for_single_word() {
        assert_eq!(join_nonempty(&["", "rust", ""]), "rust");
    }
}
