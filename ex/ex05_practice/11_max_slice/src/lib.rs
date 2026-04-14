// slice の最大値を返す課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `max_slice`
// - 引数: `xs: &[i32]`
// - 戻り値: `Option<i32>`
//
// 条件:
// - 空なら `None`
// - 要素があれば最大値を `Some(...)` で返す
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn max_slice(xs: &[i32]) -> Option<i32> {
    if xs.is_empty() {
        return None;
    }
    let mut max = i32::MIN;
    let mut it = xs.iter();

    while let Some(x) = it.next() {
        if *x > max {
            max = *x;
        }
    }
    Some(max)
}

pub fn max_slice2(xs: &[i32]) -> Option<i32> {
    xs.iter().copied().max()
}

#[cfg(test)]
mod tests {
    use super::max_slice;

    #[test]
    fn returns_none_for_empty_slice() {
        assert_eq!(max_slice(&[]), None);
    }

    #[test]
    fn returns_max_value() {
        assert_eq!(max_slice(&[3, 9, 1, 7]), Some(9));
    }

    #[test]
    fn handles_negative_values() {
        assert_eq!(max_slice(&[-8, -3, -10]), Some(-3));
    }
}
