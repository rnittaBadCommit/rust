// `Vec<i32>` の末尾を安全に取り出す課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `take_last`
// - 引数: `v: &mut Vec<i32>`
// - 戻り値: `Option<i32>`
//
// 条件:
// - 末尾要素があれば取り出して返す
// - 空なら `None`
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn take_last(v: &mut Vec<i32>) -> Option<i32> {
    Some(v.pop())?
}

#[cfg(test)]
mod tests {
    use super::take_last;

    #[test]
    fn pops_last_value() {
        let mut v = vec![10, 20, 30];
        assert_eq!(take_last(&mut v), Some(30));
        assert_eq!(v, vec![10, 20]);
    }

    #[test]
    fn returns_none_for_empty_vec() {
        let mut v = Vec::new();
        assert_eq!(take_last(&mut v), None);
    }
}
