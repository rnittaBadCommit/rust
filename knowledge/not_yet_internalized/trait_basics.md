# Trait Basics

このノートは、`knowledge/traits_generics_and_iterators.md` の trait 周りを読んだあとに、
まだ曖昧になりやすい点を切り出して整理するためのものです。

## まず一言で

- `trait` は「その型に何ができるか」の約束
- `impl Trait for Type` は「その型がその約束をどう満たすか」の定義
- `T: Trait` は「ジェネリクス `T` にその約束を要求する」という条件

## C と比べると何に近いか

C には Rust の trait と完全に同じ仕組みはありません。
まずは次の感覚で十分です。

- `struct Point { ... }` は「データの形」
- `trait Add { ... }` は「この型ならこういう操作ができる、という約束」
- `impl Add for Point { ... }` は「`Point` でその約束を満たす具体的な実装」

`void *` で何でも受けるのとは違って、
Rust は「何ができる型だけを受けるか」を型で先に書きます。

## `T: Debug` が言っていること

```rust
fn print_debug<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
```

ここで大事なのは、
`T` は「何でもできる型」ではなく「まだ具体的には決まっていない型」だという点です。

この関数の中で `x` に対して使ってよいのは、
`Debug` が保証してくれることだけです。

- `println!("{x:?}")` はできる
- `x + x` は `Add` 制約がないのでできない
- `x > y` は `PartialOrd` 制約がないのでできない

つまり `T: Debug` は、
「`T` なら何でもよい」ではなく
「`Debug` を実装している `T` だけ受け付ける」です。

## `trait bound` は実行時チェックではない

`T: Debug` や `T: Add<Output = T>` は、
関数の中で毎回調べる条件ではありません。
コンパイル時に
「この具体的な型はその trait を実装しているか」
を Rust が確認します。

C の `void *` みたいに受け取ってから中で頑張るのではなく、
呼び出せる型を先に絞るイメージです。

## `T: Add<Output = T>` の読み方

```rust
use std::ops::Add;

fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}
```

これは次の意味です。

- `T` は `Add` を実装している
- `a + b` が書ける
- その結果の型は `T`

`Output = T` は関連型の指定です。
`Add` trait 側が「加算結果の型」を `Output` という名前で持っていて、
ここではそれが `T` だと要求しています。

例えば `Point + Point` の結果を `Point` にしたいなら自然ですが、
理屈の上では「入力は `T`、結果は別の型 `U`」という設計もできます。

## なぜ `add_one<T>` はそのままでは書けないのか

```rust
fn add_one<T>(x: T) -> T {
    x + 1
}
```

これがそのまま書けない理由は 2 つあります。

1. すべての `T` が `+` を持つわけではない
2. `1` を `T` としてどう作るかが決まっていない

`T: Add<Output = T>` を付けても、
まだ「`1` を `T` にする方法」がありません。
なので最初は `add(a, b)` のように
同じ型の値を 2 つ受けて足す例で考える方が分かりやすいです。

## `impl Point` と `impl Add for Point` の違い

これは混ざりやすいところです。

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}
```

これは `Point` 自身のメソッドを定義しています。

一方で:

```rust
use std::ops::Add;

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
```

こちらは `Point` に「標準ライブラリの `Add` trait を実装している」状態を与えています。

違いを一言で言うと:

- `impl Point { ... }`: `Point` 固有のメソッドを生やす
- `impl Add for Point { ... }`: `Point` が既存 trait の約束を満たすようにする

## `impl Add for Point` の各行の意味

```rust
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
```

- `impl Add for Point`
  `Point` に対して `Add` を実装する
- `type Output = Point`
  `p1 + p2` の結果型を `Point` にする
- `fn add(self, rhs: Point) -> Point`
  `+` の実際の中身を定義する

ここでの対応はこう考えると分かりやすいです。

```rust
let p3 = p1 + p2;
```

は感覚的には:

```rust
let p3 = Add::add(p1, p2);
```

に近いです。

## `self` と `rhs` が値なので何が起きるか

`fn add(self, rhs: Point) -> Point` では、
左辺も右辺も値として受け取っています。
つまりこの実装では `p1` と `p2` は消費されます。

```rust
let p3 = p1 + p2;
// ここで p1, p2 はもう使えない
```

C でいうと、ポインタではなく値渡しの関数に近いです。

```c
Point add(Point lhs, Point rhs);
```

のような感覚で読むとよいです。

## 今の段階での理解のゴール

この段階では、まず次を言えれば十分です。

- trait は「型にできる操作の約束」
- `T: Trait` は「その操作ができる型だけを受ける」という条件
- `impl Trait for Type` は「その型でその操作をどう実装するか」
- `impl Type { ... }` とは役割が違う
- `Add<Output = T>` の `Output` は関連型

`trait object` や動的ディスパッチは、
この基礎が固まってからで十分です。
