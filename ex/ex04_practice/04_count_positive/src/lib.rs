// `&[i32]` を読むだけで集計する課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `count_positive`
// - 引数: `xs: &[i32]`
// - 戻り値: `usize`
//
// 条件:
// - 0 より大きい要素の個数を返す
// - 新しい `Vec` は作らない
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn count_positive(xs: &[i32]) -> usize {
    let mut ret: usize = 0;
    for x in xs {
        if *x > 0 {
            ret += 1;
        }
    }
    ret
}

pub fn count_positive2(xs: &[i32]) -> usize {
    xs.iter().filter(|x| **x > 0).count()
} // もう少しいいやり方ないのかな

#[cfg(test)]
mod tests {
    use super::count_positive;

    #[test]
    fn counts_only_positive_values() {
        let xs = [-3, 0, 4, 9, -1];
        assert_eq!(count_positive(&xs), 2);
    }

    #[test]
    fn handles_empty_slice() {
        let xs: [i32; 0] = [];
        assert_eq!(count_positive(&xs), 0);
    }
}
