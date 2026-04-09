# モジュールパスと `use`

このノートは、`std::fmt::Debug`、`std::ops::Add`、`std::io::Error`、`use std::fs;` のような書き方を読むための前提を整理するものです。

## まず一言で

- `std::io::Error` のような形は「どこにある名前か」を表すパス
- `use` は、その長い名前を今のスコープで短く使えるようにする宣言
- `use` はコードをコピーするのではなく、「この名前で参照する」と決めるだけ

## `std::io::Error` はどう読むか

```rust
std::io::Error
```

は、ざっくり次の意味です。

- `std`: 標準ライブラリ
- `io`: その中の `io` モジュール
- `Error`: その中にある型

つまり「標準ライブラリの `io` モジュールにある `Error` 型」です。

C でたとえるなら、
ヘッダを `#include` したあとに
「どのライブラリ由来の名前か」を区別しながら読んでいる感覚に少し近いです。
ただし Rust は名前空間をパスでより明示的に表します。

## `use` は何をするのか

例えば:

```rust
use std::fs;

fn main() {
    let text = fs::read_to_string("hello.txt");
    println!("{text}");
}
```

ここでの `use std::fs;` は、
`std::fs` という名前をこのスコープで `fs` として使えるようにしています。

もし `use` がなければ、こう書けます。

```rust
fn main() {
    let text = std::fs::read_to_string("hello.txt");
    println!("{text}");
}
```

意味は同じです。
違うのは、毎回フルパスを書くか、先に `use` して短く書くかだけです。

## trait 名でも同じ

例えば:

```rust
use std::ops::Add;

fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}
```

ここでの `Add` は、
本当は `std::ops::Add` です。

`use` がなければ次のようにも書けます。

```rust
fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}
```

## `std::slice::Iter<'a, i32>` のような長い名前

`for` やイテレータ周りでは、こういう型名が出ます。

```rust
std::slice::Iter<'a, i32>
```

読み方は同じです。

- `std`
- `slice`
- `Iter`

の順にたどって、
「標準ライブラリの `slice` モジュールにある `Iter` 型」と読めば十分です。

最初は中身まで覚える必要はありません。
まずは「長い名前は、どこにあるものかを表す住所みたいなもの」と思えばよいです。

## C と比べると

C では、同じ `open`, `read`, `error` のような短い名前が
どこ由来なのか文脈に依存しやすいです。

Rust は、

- `std::fs::read_to_string`
- `std::io::Error`
- `std::fmt::Debug`

のように、
名前の所属をパスで明示しやすいです。

その上で `use` を使うと、読みやすさのために短くできます。

## 今の段階での理解のゴール

- `a::b::C` は「`a` の中の `b` の中の `C`」
- `use a::b::C;` は、その名前を今の場所で短く使うための宣言
- `use` がなくてもフルパスで書けば同じ意味になることが多い
- `std::ops::Add` も `std::io::Error` も、基本は同じ「名前の住所」
