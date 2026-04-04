#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
}

/// 値の種類に応じて説明文字列を返します。
///
/// Cなら `enum + union` にタグを持たせる形に近いですが、
/// Rustでは `match` で全パターンを安全に処理できます。
#[allow(unused_variables)]
pub fn describe(value: &Value) -> String {
    todo!("`match` を使って値を説明する文字列を返してください");
}

/// Intなら 1 加算し、Floatなら 1.0 加算して返します。
#[allow(unused_variables)]
pub fn add_one(value: Value) -> Value {
    todo!("`match` を使って値に応じた加算をしてください");
}

#[cfg(test)]
mod tests {
    use super::{add_one, describe, Value};

    #[test]
    fn describe_int() {
        assert_eq!(describe(&Value::Int(7)), "int: 7");
    }

    #[test]
    fn describe_float() {
        assert_eq!(describe(&Value::Float(3.5)), "float: 3.5");
    }

    #[test]
    fn add_one_updates_int() {
        assert_eq!(add_one(Value::Int(10)), Value::Int(11));
    }

    #[test]
    fn add_one_updates_float() {
        assert_eq!(add_one(Value::Float(2.5)), Value::Float(3.5));
    }
}
