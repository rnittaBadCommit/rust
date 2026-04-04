use std::io;

/// 空行と空白だけの行を除いて、ファイル中の行数を数える課題です。
///
/// 実装方針の例:
/// - `std::fs::read_to_string` を使う
/// - `?` で I/O エラーをそのまま返す
/// - `trim()` してから空かどうかを見る
#[allow(unused_variables)]
pub fn count_nonempty_lines(path: &str) -> Result<usize, io::Error> {
    todo!("ファイルを読んで、空行と空白行を除いた行数を返してください");
}

#[cfg(test)]
mod tests {
    use super::count_nonempty_lines;

    #[test]
    fn counts_only_nonempty_lines() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/sample.txt");
        assert_eq!(count_nonempty_lines(path).unwrap(), 3);
    }

    #[test]
    fn missing_file_returns_error() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/does_not_exist.txt");
        assert!(count_nonempty_lines(path).is_err());
    }
}

