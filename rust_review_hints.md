# Rust Review Hints

`rust_review_questions.md` 用のヒントです。解答ではなく、考えるための足場だけを書いています。

## 1. `&str` とスライス

- `line.find('=')` を使うと、`=` の位置を `Option<usize>` で取れます。
- 添字 `idx` が見つかったら、前半は `&line[..idx]`、後半は `&line[idx + 1..]` の形で取れます。
- 前半か後半が空文字列なら `None` にします。
- 戻り値の `&str` は、入力の `line` を借りているだけです。新しい所有文字列は不要です。

## 2. 可変借用と `Vec`

- `for x in &mut *xs` または `for x in xs.iter_mut()` の形で、各要素を `&mut i32` として触れます。
- `*x` を見て、0未満なら `*x = 0`、100超なら `*x = 100` にします。
- 「新しい `Vec` を返す」のではなく、「借りた `Vec` の中身を変える」問題です。

## 3. `struct` と `impl`

- `Stack` の中身は `items: Vec<i32>` のようにすると素直です。
- `new` は空の `Vec` を持つ `Stack` を返せば十分です。
- `push` は `self.items.push(value)`。
- `pop` は `self.items.pop()` をそのまま使えます。
- `peek` は `self.items.last()` が便利です。返り値の型は `Option<&i32>` です。
- `len` は `self.items.len()`。
- どのメソッドが `&self`、`&mut self` を取るべきかを意識してください。

## 4. `enum` と `match`

- `describe` は `&Packet` を受け取るので、`match p { ... }` で分岐するときも基本は借用のまま扱います。
- `Message(text)` のケースでは、`text` は `&String` として見えます。
- 文字列を返す必要があるので、`format!` を使うと書きやすいです。
- `Move { x, y }` のように、構造体風バリアントも分解できます。

## 5. `Result` とファイルI/O

- まず `std::fs::read_to_string(path)?` で内容全体を読み込む形が簡単です。
- `lines()` で1行ずつ見られます。
- 空白だけの行を除外したいので、各行に `trim()` をかけてから空かどうかを見ます。
- 件数を数える方法は、最初は `let mut count = 0;` で十分です。
- `main` でも `?` を使いたいので、`fn main() -> Result<(), std::io::Error>` にします。

