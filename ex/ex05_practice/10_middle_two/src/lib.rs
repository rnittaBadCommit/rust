// 4 要素配列から真ん中 2 要素を slice として返す課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `middle_two`
// - 引数: `xs: &[i32; 4]`
// - 戻り値: `&[i32]`
//
// 条件:
// - 真ん中 2 要素だけを返す
// - 新しい配列や `Vec` は作らない
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn middle_two(xs: &[i32; 4]) -> &[i32] {
    &xs[1..3]
}

#[cfg(test)]
mod tests {
    use super::middle_two;

    #[test]
    fn returns_middle_two_elements() {
        let xs = [10, 20, 30, 40];
        assert_eq!(middle_two(&xs), &[20, 30]);
    }

    #[test]
    fn works_with_negative_values() {
        let xs = [-5, -1, 7, 9];
        assert_eq!(middle_two(&xs), &[-1, 7]);
    }
}
