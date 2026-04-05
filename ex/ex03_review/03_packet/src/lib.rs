#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Ping,
    Message(String),
    Move { x: i32, y: i32 },
}

/// `Packet` を人が読める説明文字列に変換する課題です。
///
/// 条件:
/// - `match` を使う
/// - `&Packet` を受け取り、借用のまま読む
#[allow(unused_variables)]
pub fn describe(p: &Packet) -> String {
    // todo!("`Packet` の種類ごとに説明文字列を返してください");
    match p {
        Packet::Ping => String::from("ping"),
        Packet::Message(message) => String::from("msg: ") + message,
        Packet::Move { x, y } => {
            return String::from("move to (") // 練習（検証）のために、ここでreturnでも行けるのか試してみる
                + x.to_string().as_str()
                + ", "
                + y.to_string().as_str()
                + ")";
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Packet, describe};

    #[test]
    fn describes_ping() {
        assert_eq!(describe(&Packet::Ping), "ping");
    }

    #[test]
    fn describes_message() {
        let packet = Packet::Message(String::from("hi"));
        assert_eq!(describe(&packet), "msg: hi");
    }

    #[test]
    fn describes_move() {
        let packet = Packet::Move { x: 3, y: -1 };
        assert_eq!(describe(&packet), "move to (3, -1)");
    }
}
