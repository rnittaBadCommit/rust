use std::io;

/// ファイルを読んで行数を返します。
///
/// 実装方針の例:
/// - `std::fs::read_to_string` を使う
/// - `?` で I/O エラーをそのまま返す
/// - `text.lines().count()` で行数を数える
#[allow(unused_variables)]
pub fn count_lines(path: &str) -> Result<usize, io::Error> {
    todo!("ファイルを読んで行数を返してください");
}

#[cfg(test)]
mod tests {
    use super::count_lines;

    #[test]
    fn counts_lines_in_fixture_file() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/sample.txt");
        assert_eq!(count_lines(path).unwrap(), 3);
    }

    #[test]
    fn missing_file_returns_error() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/does_not_exist.txt");
        assert!(count_lines(path).is_err());
    }
}
