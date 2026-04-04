/// スコアの配列をその場で補正する課題です。
///
/// 仕様:
/// - 0未満は0にする
/// - 100超は100にする
/// - それ以外はそのままにする
/// - 新しい `Vec` は作らない
#[allow(unused_variables)]
pub fn clamp_scores(xs: &mut Vec<i32>) {
    todo!("`xs` の各要素をその場で補正してください");
}

#[cfg(test)]
mod tests {
    use super::clamp_scores;

    #[test]
    fn keeps_values_in_range() {
        let mut xs = vec![0, 10, 55, 100];
        clamp_scores(&mut xs);
        assert_eq!(xs, vec![0, 10, 55, 100]);
    }

    #[test]
    fn clamps_values_outside_range() {
        let mut xs = vec![-10, 30, 120, -1, 101];
        clamp_scores(&mut xs);
        assert_eq!(xs, vec![0, 30, 100, 0, 100]);
    }

    #[test]
    fn handles_empty_vector() {
        let mut xs = Vec::new();
        clamp_scores(&mut xs);
        assert!(xs.is_empty());
    }
}

