// `Vec<String>` から空文字列だけを取り除く課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `drop_empty`
// - 引数: `v: &mut Vec<String>`
// - 戻り値: なし
//
// 条件:
// - 空文字列だけを取り除く
// - 残りの順序は保つ
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

#[cfg(test)]
mod tests {
    use super::drop_empty;

    #[test]
    fn removes_empty_strings_and_keeps_order() {
        let mut v = vec![
            String::from(""),
            String::from("alpha"),
            String::from(""),
            String::from("beta"),
            String::from("gamma"),
        ];
        drop_empty(&mut v);
        assert_eq!(v, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn can_remove_every_element() {
        let mut v = vec![String::from(""), String::from("")];
        drop_empty(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn leaves_nonempty_vec_unchanged() {
        let mut v = vec![String::from("rust"), String::from("borrow")];
        drop_empty(&mut v);
        assert_eq!(v, vec!["rust", "borrow"]);
    }
}
