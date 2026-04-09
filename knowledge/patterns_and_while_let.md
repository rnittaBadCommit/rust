# パターン束縛と `while let`

このノートは、`Some(v)`、`Ok(text)`、`Value::Int(i)`、`for (i, x)`、`while let Some(x) = ...` のような書き方をまとめて理解するためのものです。

## まず一言で

- パターンは「値の形」
- Rust はその形に合わせて値を分解し、中身に名前を付けられる
- `while let` は「その形に合う間だけ繰り返す」構文

## パターンとは何か

例えば:

```rust
match v {
    Value::Int(i) => println!("{i}"),
    Value::Float(f) => println!("{f}"),
}
```

ここでの

- `Value::Int(i)`
- `Value::Float(f)`

がパターンです。

意味は、

- `v` が `Value::Int(...)` という形なら、中の値を `i` として取り出す
- `v` が `Value::Float(...)` という形なら、中の値を `f` として取り出す

です。

## `Some(v)` や `Ok(text)` も同じ

例えば:

```rust
match find_even(&a) {
    Some(v) => println!("found: {v}"),
    None => println!("not found"),
}
```

```rust
match read_file("hello.txt") {
    Ok(text) => println!("{text}"),
    Err(err) => println!("{err}"),
}
```

ここでも同じです。

- `Some(v)` は「`Some` の中身を `v` として取り出す」
- `Ok(text)` は「`Ok` の中身を `text` として取り出す」
- `Err(err)` は「`Err` の中身を `err` として取り出す」

## タプルも分解できる

```rust
for (i, x) in v.iter().enumerate() {
    println!("{i} {x}");
}
```

ここで `enumerate()` が返す要素は、
`(添字, 要素)` という 2 要素のタプルです。

だから `(i, x)` というパターンで受けると、

- 1 個目を `i`
- 2 個目を `x`

として分解できます。

## `let` でもパターンを使っている

```rust
let (a, b) = (10, 20);
```

これも同じ発想です。
右辺のタプルを左辺の形 `(a, b)` で受けて分解しています。

## `while let` は何か

例えば:

```rust
while let Some(x) = it.next() {
    println!("{x}");
}
```

これは、
`it.next()` の結果が `Some(x)` という形に合う間は繰り返し、
合わなくなったら止める、
という意味です。

`Iterator::next()` は普通、

- 要素があれば `Some(要素)`
- 終端なら `None`

を返します。

だからこの `while let` は
「次の要素がある間、1 個ずつ取り出す」
と読めます。

## `while let` を `match` っぽく展開すると

感覚としては次に近いです。

```rust
loop {
    match it.next() {
        Some(x) => {
            println!("{x}");
        }
        None => break,
    }
}
```

つまり `while let` は、
よくある「一致したら続行、一致しなければ終了」という形を短く書く道具です。

## C と比べると

C だと、

- タグを見て分岐する
- union や struct の中身を取り出す
- `NULL` かどうかを確認する

といった処理を別々に書くことが多いです。

Rust はパターンで
「どの形かの確認」と「中身の取り出し」をまとめて書けます。

## 今の段階での理解のゴール

- `Some(v)` や `Ok(text)` は「形を見て中身を取り出す」書き方
- `Value::Int(i)` も同じ種類のパターン
- `(i, x)` はタプルを分解するパターン
- `while let Some(x) = ...` は、その形に合う間だけ続ける
