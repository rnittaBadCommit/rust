// 末尾要素を読んでから push する順序に慣れる課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `duplicate_last`
// - 引数: `v: &mut Vec<i32>`
// - 戻り値: なし
//
// 条件:
// - 末尾要素があれば同じ値を 1 個追加する
// - 空なら何もしない
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn duplicate_last(v: &mut Vec<i32>) {
    if let Some(last) = v.last() {
        v.push(*last);
    }
}

#[cfg(test)]
mod tests {
    use super::duplicate_last;

    #[test]
    fn duplicates_last_value() {
        let mut v = vec![1, 2, 3];
        duplicate_last(&mut v);
        assert_eq!(v, vec![1, 2, 3, 3]);
    }

    #[test]
    fn leaves_empty_vec_unchanged() {
        let mut v = Vec::new();
        duplicate_last(&mut v);
        assert!(v.is_empty());
    }
}
