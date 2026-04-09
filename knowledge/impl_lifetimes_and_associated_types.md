# `impl<'a>` と関連型

このノートは、`impl<'a> IntoIterator for &'a Bag`、`type IntoIter = ...`、`Self::IntoIter` の読み方を整理するものです。

## まず一言で

- `impl<'a>` は「任意のライフタイム `'a` についてこの実装がある」という意味
- 関連型は、trait が持つ「この trait にぶら下がる型の名前」
- `Self::IntoIter` は「この実装で決めた `IntoIter` 型」

## `impl<'a>` はどう読むか

例えば:

```rust
impl<'a> IntoIterator for &'a Bag {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
```

ここでの `<'a>` は、
関数の

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

に出てくる `'a` と同じ種類のライフタイム引数です。

違うのは、

- 関数に付いているか
- `impl` に付いているか

だけです。

この `impl<'a>` は、
「どんなライフタイム `'a` の `&Bag` に対しても、この `IntoIterator` 実装を使える」
と読めば十分です。

## `for &'a Bag` とは何を意味するか

```rust
impl<'a> IntoIterator for &'a Bag
```

は、
`Bag` そのものではなく
`Bag` への参照 `&'a Bag` に対して `IntoIterator` を実装している、
という意味です。

だから `for x in &bag` が動きます。

同様に:

```rust
impl<'a> IntoIterator for &'a mut Bag
```

なら `for x in &mut bag` が動きます。

## 関連型とは何か

trait は、メソッドだけでなく
「その trait に関係する型の名前」も持てます。
それが関連型です。

例えば `IntoIterator` には、
概念的に次の 2 つがあります。

- `Item`: 1 回ごとに取り出す要素の型
- `IntoIter`: 実際のイテレータ本体の型

だから実装側で:

```rust
type Item = &'a i32;
type IntoIter = std::slice::Iter<'a, i32>;
```

のように具体化します。

## `Output` も同じ種類

すでに見た

```rust
T: Add<Output = T>
```

の `Output` も関連型です。

つまり:

- `Add` では加算結果の型を `Output` と呼ぶ
- `IntoIterator` では反復要素を `Item`、反復本体を `IntoIter` と呼ぶ

というだけで、仕組みは同じです。

## `Self::IntoIter` は何か

```rust
fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
}
```

この `Self` は、
その `impl` の対象型を指します。

ここでの戻り値 `Self::IntoIter` は、
「この実装で定義した `IntoIter` 関連型」
という意味です。

この例では実質的に次と同じです。

```rust
fn into_iter(self) -> std::slice::Iter<'a, i32> {
    self.0.iter()
}
```

ただし `Self::IntoIter` と書く方が、
trait の約束に沿って読めるので自然です。

## `type IntoIter = ...` と `fn into_iter(...)` の関係

この 2 行はつながっています。

```rust
type IntoIter = std::slice::Iter<'a, i32>;

fn into_iter(self) -> Self::IntoIter
```

意味は、

- `IntoIter` という関連型の正体は `std::slice::Iter<'a, i32>`
- `into_iter` はその型の値を返す

です。

## C と比べると

C には trait も関連型もないので完全対応はありません。

感覚としては、

- `impl<'a>` は「この寿命の参照でも使える」という型レベルの宣言
- 関連型は「このインターフェースに付属する型名」

と思うとよいです。

関数ポインタや struct を別々に渡すより、
Rust は「その trait に必要な型情報」をひとまとめで表せます。

## 今の段階での理解のゴール

- `impl<'a>` はライフタイム引数つきの実装
- `impl<'a> IntoIterator for &'a Bag` は `&Bag` 用の実装
- `type Item = ...` や `type IntoIter = ...` は関連型の具体化
- `Self::IntoIter` は、その実装で決めた `IntoIter`
- `Output`、`Item`、`IntoIter` は全部「trait に属する型名」という点で同じ
