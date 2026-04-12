// `&mut String` を受け取り、その場で末尾を調整する課題です。
//
// 自分で次の宣言から書いてください。
//
// - 関数名: `append_exclamation`
// - 引数: `s: &mut String`
// - 戻り値: なし
//
// 条件:
// - 末尾が `!` でなければ `!` を 1 個だけ足す
// - すでに `!` で終わっているならそのまま
//
// テストは通常のコードとして置いてあります。
// 先に関数宣言を書いてから `cargo test` してください。

pub fn append_exclamation(s: &mut String) {
    if let Some(last_char) = s.chars().last() {
        if last_char == '!' {
            s.push('!');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::append_exclamation;

    #[test]
    fn appends_exclamation_when_missing() {
        let mut s = String::from("hello");
        append_exclamation(&mut s);
        assert_eq!(s, "hello!");
    }

    #[test]
    fn does_not_append_twice() {
        let mut s = String::from("wow!");
        append_exclamation(&mut s);
        assert_eq!(s, "wow!");
    }
}
