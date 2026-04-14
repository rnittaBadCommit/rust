// 長方形の面積と正方形判定を書く課題です。
//
// 自分で次の `struct` と `impl` を書いてください。
//
// - `struct Rectangle { width: u32, height: u32 }`
// - `area(&self) -> u32`
// - `is_square(&self) -> bool`
//
// 条件:
// - 面積は `width * height`
// - `width == height` なら正方形
//
// テストは通常のコードとして置いてあります。
// 先に `struct` と `impl` を書いてから `cargo test` してください。

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn computes_area() {
        let rect = Rectangle {
            width: 3,
            height: 4,
        };
        assert_eq!(rect.area(), 12);
    }

    #[test]
    fn detects_square() {
        let rect = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(rect.is_square());
    }

    #[test]
    fn detects_non_square() {
        let rect = Rectangle {
            width: 5,
            height: 4,
        };
        assert!(!rect.is_square());
    }
}
