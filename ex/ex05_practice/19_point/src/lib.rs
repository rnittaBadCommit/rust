// 座標を更新する型を書く課題です。
//
// 自分で次の `struct` と `impl` を書いてください。
//
// - `struct Point { x: i32, y: i32 }`
// - `translate(&mut self, dx: i32, dy: i32)`
// - `manhattan_len(&self) -> i32`
//
// 条件:
// - `translate` は座標をその場で更新する
// - `manhattan_len` は `|x| + |y|` を返す
//
// テストは通常のコードとして置いてあります。
// 先に `struct` と `impl` を書いてから `cargo test` してください。

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn translate_updates_both_coordinates() {
        let mut p = Point { x: 1, y: 2 };
        p.translate(3, -5);
        assert_eq!(p.x, 4);
        assert_eq!(p.y, -3);
    }

    #[test]
    fn computes_manhattan_length() {
        let p = Point { x: -3, y: 4 };
        assert_eq!(p.manhattan_len(), 7);
    }

    #[test]
    fn translate_and_length_work_together() {
        let mut p = Point { x: 0, y: 0 };
        p.translate(-2, 5);
        assert_eq!(p.manhattan_len(), 7);
    }
}
