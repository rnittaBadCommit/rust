// 可変 slice をその場で書き換える課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `negate_all`
// - 引数: `xs: &mut [i32]`
// - 戻り値: なし
//
// 条件:
// - すべての要素を符号反転する
// - 新しい `Vec` は作らない
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn negate_all(xs: &mut [i32]) {
    for x in xs {
        *x *= -1;
    }
}

pub fn negate_all2(xs: &mut [i32]) {
    // count_positiveのように1行で出来ないのかな？
}

#[cfg(test)]
mod tests {
    use super::negate_all;

    #[test]
    fn negates_each_element() {
        let mut xs = [1, -2, 0, 7];
        negate_all(&mut xs);
        assert_eq!(xs, [-1, 2, 0, -7]);
    }

    #[test]
    fn handles_empty_slice() {
        let mut xs: [i32; 0] = [];
        negate_all(&mut xs);
        assert_eq!(xs, []);
    }
}
