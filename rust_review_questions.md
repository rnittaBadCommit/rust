# Rust Review Questions

`rust_from_c_guide.md` のうち、`trait` 以外を復習するための問題です。
所有権、借用、スライス、`String` と `&str`、`struct`、`enum`、`match`、`Option`、`Result`、`Vec` を中心にしています。

## 1. `&str` とスライス

次の関数を書いてください。

```rust
fn split_key_value(line: &str) -> Option<(&str, &str)>
```

仕様:

- `"name=Alice"` なら `Some(("name", "Alice"))`
- `"name="` は `None`
- `"=Alice"` は `None`
- `"name"` は `None`
- 最初の `=` だけで分割する
- 新しい `String` は作らない

## 2. 可変借用と `Vec`

次の関数を書いてください。

```rust
fn clamp_scores(xs: &mut Vec<i32>)
```

仕様:

- 各要素について、0未満は0にする
- 100超は100にする
- それ以外はそのままにする
- 新しい `Vec` は作らず、その場で書き換える

## 3. `struct` と `impl`

`Vec<i32>` を内部に持つ `struct Stack` を作ってください。

実装するもの:

- `new`
- `push`
- `pop`
- `peek`
- `len`

条件:

- `pop` は `Option<i32>` を返す
- `peek` は `Option<&i32>` を返す
- `main` で `10`, `20`, `30` を積み、空になるまで取り出して表示する

## 4. `enum` と `match`

次の列挙型を作ってください。

```rust
enum Packet {
    Ping,
    Message(String),
    Move { x: i32, y: i32 },
}
```

そのうえで、次の関数を書いてください。

```rust
fn describe(p: &Packet) -> String
```

仕様:

- `Ping` は `"ping"`
- `Message("hi")` は `"msg: hi"`
- `Move { x: 3, y: -1 }` は `"move to (3, -1)"`
- `match` を必ず使う

## 5. `Result` とファイルI/O

次の関数を書いてください。

```rust
fn count_nonempty_lines(path: &str) -> Result<usize, std::io::Error>
```

仕様:

- ファイルを読む
- 空行と、空白だけの行は数えない
- それ以外の行数を返す

条件:

- `main() -> Result<(), std::io::Error>` の形にする
- `?` を使って呼び出す

