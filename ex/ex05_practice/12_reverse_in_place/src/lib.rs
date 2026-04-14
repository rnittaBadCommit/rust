// 可変 slice をその場で反転する課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `reverse_in_place`
// - 引数: `xs: &mut [i32]`
// - 戻り値: なし
//
// 条件:
// - 新しい `Vec` は作らない
// - 与えられた slice 自体を書き換える
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn reverse_in_place(xs: &mut [i32]) {
    if xs.len() < 2 {
        return;
    }
    let i_middle = xs.len() / 2;
    let mut i_left = 0 * i_middle;
    let mut i_right = xs.len() - 1;

    while i_left < i_middle {
        let tmp = xs[i_left];
        xs[i_left] = xs[i_right];
        xs[i_right] = tmp;
        i_left += 1;
        i_right += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_in_place;

    #[test]
    fn reverses_even_length_slice() {
        let mut xs = [1, 2, 3, 4];
        reverse_in_place(&mut xs);
        assert_eq!(xs, [4, 3, 2, 1]);
    }

    #[test]
    fn reverses_odd_length_slice() {
        let mut xs = [1, 2, 3, 4, 5];
        reverse_in_place(&mut xs);
        assert_eq!(xs, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn leaves_single_element_slice_unchanged() {
        let mut xs = [42];
        reverse_in_place(&mut xs);
        assert_eq!(xs, [42]);
    }
}
