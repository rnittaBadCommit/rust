# `char_indices()` と文字列の切り出し

このノートは、`&str` を走査しながら一部を切り出したいときに、
なぜ `chars().enumerate()` ではなく `char_indices()` を使うのかを整理するためのものです。

## まず一言で

- `char_indices()` は「文字」と「その文字が始まるバイト位置」を返す
- `&str` のスライス `&s[a..b]` に使う添字は、文字番号ではなくバイト位置
- 空白判定を Unicode 対応で行いたいなら、`ch.is_whitespace()` と組み合わせると自然

## `chars().enumerate()` では何が足りないのか

```rust
for (i, ch) in s.chars().enumerate() {
    // ...
}
```

この `i` は「何文字目か」です。
これは `&s[a..b]` にそのまま使える値ではありません。

Rust の `&str` は UTF-8 なので、
1 文字が 1 バイトとは限りません。

たとえば:

```rust
let s = "aé";
```

- `'a'` は 1 バイト
- `'é'` は 2 バイト

です。

このとき:

- 文字番号としては `'a'` が 0、`'é'` が 1
- バイト位置としては `'a'` が 0、`'é'` が 1
- 文字列末尾のバイト位置は 3

つまり「文字番号」と「スライスに使う添字」は別物です。

## `char_indices()` は何を返すのか

```rust
for (i, ch) in s.char_indices() {
    // ...
}
```

ここでの `i` は、「その文字が始まるバイト位置」です。

だから `i` はそのまま:

```rust
&s[start..i]
```

のようなスライスに使えます。

## `is_whitespace()` と組み合わせる例

```rust
fn first_word(s: &str) -> Option<&str> {
    let mut start = None;

    for (i, ch) in s.char_indices() {
        if ch.is_whitespace() {
            if let Some(start) = start {
                return Some(&s[start..i]);
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }

    start.map(|start| &s[start..])
}
```

この例で大事なのは次です。

- 文字の種類を見るのは `ch.is_whitespace()`
- 切り出し位置を覚えるのは `char_indices()` の `i`

`is_whitespace()` を使うので、
ASCII の `' '` や `'\t'` だけでなく、
Unicode の空白もまとめて扱えます。

## C と比べると

C なら感覚としては:

```c
while (isspace((unsigned char)*p)) p++;
start = p;
while (*p && !isspace((unsigned char)*p)) p++;
```

に近いです。

ただし C の `char *` は基本的にバイト列を直接たどりますが、
Rust の `&str` は UTF-8 文字列なので、
「何文字目か」と「何バイト目か」を分けて考える必要があります。

そのため Rust では:

- 文字として判定する: `ch.is_whitespace()`
- スライス位置を取る: `char_indices()`

と役割を分けると分かりやすいです。

## まずはこう覚える

- `chars()` は文字を順に見る
- `chars().enumerate()` の添字は文字番号
- `char_indices()` の添字はバイト位置
- `&str` を安全に切り出したいときは、まず `char_indices()` を考える

## 今の段階での理解のゴール

- `&str` のスライスに使う添字は、文字番号ではなくバイト位置
- だから `chars().enumerate()` より `char_indices()` が向いている場面がある
- 空白や区切り文字で単語を切るときは、`char_indices()` と `is_whitespace()` の組み合わせが自然
