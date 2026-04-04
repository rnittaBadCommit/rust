use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Point {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("`(x, y)` の形で表示してください");
    }
}

#[allow(unused_variables)]
pub fn render<T: fmt::Display>(value: &T) -> String {
    todo!("`T: Display` を使って文字列に変換してください");
}

#[cfg(test)]
mod tests {
    use super::{render, Point};

    #[test]
    fn point_formats_as_tuple_like_text() {
        let p = Point { x: 3, y: -4 };
        assert_eq!(format!("{p}"), "(3, -4)");
    }

    #[test]
    fn generic_render_works_for_point() {
        let p = Point { x: 10, y: 20 };
        assert_eq!(render(&p), "(10, 20)");
    }

    #[test]
    fn generic_render_works_for_builtin_display_types_too() {
        assert_eq!(render(&42), "42");
    }
}
