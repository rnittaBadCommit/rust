# 属性と `derive`

このノートは、`#[derive(Debug)]` や「`Copy` を付ける」の意味を整理するものです。

## まず一言で

- `#[...]` は属性
- `derive` は「この trait の実装を自動生成してほしい」という指定
- `Copy` を付けると言うときは、よく `#[derive(Clone, Copy)]` を意味する

## `#[derive(Debug)]` は何をしているか

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

これは、
`Point` に対して `Debug` trait の実装を自動で作ってもらう、
という意味です。

その結果、例えば次が書けます。

```rust
let p = Point { x: 1, y: 2 };
println!("{p:?}");
```

`Debug` がないと `{:?}` で表示できません。

## 属性とは何か

`#[derive(Debug)]` の `#[...]` 全体が属性です。

最初は、
「この型や関数に追加の指示を与えるメタ情報」
と思っておけば十分です。

よく見る形:

```rust
#[derive(Debug)]
struct Point { ... }
```

```rust
#[derive(Clone, Copy)]
struct Pair {
    a: i32,
    b: i32,
}
```

## `Copy` を付けるとは何か

例えば整数は `Copy` 型です。

```rust
let x = 10;
let y = x;
println!("{x} {y}");
```

`x` はそのまま使えます。
これは代入時に値がコピーされるからです。

自作型でも、条件を満たせば `Copy` にできます。

```rust
#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

こうすると:

```rust
let p1 = Point { x: 1, y: 2 };
let p2 = p1;

println!("{p1:?}");
println!("{p2:?}");
```

ができます。
`p1` はムーブされず、コピーされます。

## なぜ `Clone` も一緒に書くことが多いのか

`Copy` 型は `Clone` でもあるべき、という関係があります。
そのため実際には次のように書くことが多いです。

```rust
#[derive(Clone, Copy)]
```

最初は

- `Copy`: 代入や引数渡しで暗黙コピーされる
- `Clone`: 明示的に `.clone()` で複製できる

と理解すれば十分です。

## 何でも `Copy` にできるわけではない

例えば `String` を持つ型は普通は `Copy` にできません。

```rust
struct User {
    name: String,
}
```

`String` はヒープを所有するので、
ただのビットコピーで複製すると二重解放の危険が出ます。

そのため `Copy` にできるのは、
各フィールドも `Copy` であるような軽い値が基本です。

## `derive` できる trait と、できない trait

例えば `Debug`、`Clone`、`Copy` は `derive` しやすい代表例です。

一方で `Add` のように、
「その型ごとの具体的な振る舞い」を書かなければならない trait は、
普通は自分で `impl Add for Point` を書きます。

つまり:

- `derive` は自動で作れるもの向け
- `impl Trait for Type` は中身を自分で決めるもの向け

です。

## C と比べると

C では、
表示用関数やコピー関数を自分で毎回書くことが多いです。

Rust は trait と `derive` によって、
よくある定型実装は自動生成しやすくなっています。

## 今の段階での理解のゴール

- `#[...]` は属性
- `#[derive(Debug)]` は `Debug` 実装を自動で付ける指定
- `Copy` を付けるとは、よく `#[derive(Clone, Copy)]` のこと
- `Copy` にできるのは、各フィールドも安全にコピーできる型が基本
- `Debug` は `derive` しやすいが、`Add` のような振る舞いは普通は手で `impl` する
