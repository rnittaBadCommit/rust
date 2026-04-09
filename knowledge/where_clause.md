# `where` 句

このノートは、ジェネリクスや trait bound を読むときに出てくる `where` の書き方を整理するものです。

## まず一言で

- `where` は trait bound を別の場所に書くための構文
- 意味は「この型にはこういう条件が必要」
- 短い条件なら `<T: Debug>` でもよいが、長くなると `where` の方が読みやすい

## まずは短い形

```rust
fn print_debug<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
```

これは、
`T` は `Debug` を実装していなければならない、
という意味です。

## 同じ意味を `where` で書く

同じことをこうも書けます。

```rust
fn print_debug<T>(x: T)
where
    T: std::fmt::Debug,
{
    println!("{x:?}");
}
```

意味は同じです。
違うのは「条件をどこに置くか」だけです。

## なぜ `where` を使うのか

条件が長くなると、
関数名の横に全部書くと読みにくくなります。

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

この `where` は、
「`T` は `Add` を実装していて、加算結果も `T` である」
という条件を書いています。

## 複数条件も書ける

```rust
fn show_and_compare<T>(a: T, b: T)
where
    T: std::fmt::Debug + PartialOrd,
{
    println!("{a:?} {b:?}");

    if a > b {
        println!("a is larger");
    }
}
```

ここでは `T` に 2 つの条件があります。

- `Debug` だから `{:?}` で表示できる
- `PartialOrd` だから `>` で比較できる

## 「型引数の条件を後ろに回しただけ」と考えてよい

最初の段階では、

```rust
fn f<T>(...)
where
    T: SomeTrait,
```

を見たら、

```rust
fn f<T: SomeTrait>(...)
```

の長い版だと思って構いません。

もちろん、複雑なコードでは `where` でないと書きづらい条件も出ます。
ただ、読む側としての第一歩は
「後ろに条件がまとまっているだけ」と理解すれば十分です。

## C と比べると

C にはこれと同じ構文はありません。
感覚としては、
関数宣言の横ではなく後ろに
「この型にはこの操作が必要」
という前提条件を別欄で書いていると思うと近いです。

## 今の段階での理解のゴール

- `where` は条件を書く場所
- 条件の意味自体は trait bound と同じ
- 短い条件は `<T: Trait>`、長い条件は `where` が読みやすい
- `where` を見たら「この関数が使える型の条件」を読めばよい
