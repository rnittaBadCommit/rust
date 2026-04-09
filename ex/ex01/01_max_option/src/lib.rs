use std::i32;

/// スライスの最大値を返します。
///
/// Cなら「空なら -1 を返す」「boolで成功/失敗を返して out 引数に書く」
/// のような設計になりがちですが、Rustでは `Option<i32>` を使います。
#[allow(unused_variables)]
pub fn max_in_slice(xs: &[i32]) -> Option<i32> {
    // todo!("空なら None、そうでなければ最大値を Some(...) で返してください");
    if xs.is_empty() {
        return None;
    }
    let mut ret: i32 = i32::MIN;
    for x in xs {
        if ret < *x {
            ret = *x
        }
    }
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::max_in_slice;

    #[test]
    fn empty_slice_returns_none() {
        assert_eq!(max_in_slice(&[]), None);
    }

    #[test]
    fn returns_max_for_positive_values() {
        assert_eq!(max_in_slice(&[3, 10, 7, 1]), Some(10));
    }

    #[test]
    fn returns_max_for_negative_values_too() {
        assert_eq!(max_in_slice(&[-10, -3, -20]), Some(-3));
    }
}
