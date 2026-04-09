# For と IntoIterator

このノートは、`for x in xs`、`for x in &xs`、`for x in xs.iter()` が
誰の責任でどう動くかを切り分けて整理するためのものです。

## まず一言で

- `for` が直接見るのは `IntoIterator`
- `iter()` と `iter_mut()` は `for` の必須条件ではない
- `for x in xs.iter()` が動くのは、`iter()` の戻り値が `Iterator` であり、`Iterator` は `IntoIterator` としても使えるから

## `for` は何を呼ぶのか

```rust
for x in expr {
    body
}
```

は概念的には次です。

```rust
let mut it = IntoIterator::into_iter(expr);

while let Some(x) = it.next() {
    body
}
```

つまり `for` が最初に呼ぶのは常に `IntoIterator::into_iter(expr)` です。
`for` が `iter()` や `iter_mut()` を特別扱いして呼ぶわけではありません。

## `for` は言語機能なのに、なぜライブラリを使うのか

`for` の構文自体は Rust 言語の組み込みです。
パーサもコンパイラも `for pat in expr { ... }` という形を特別に理解しています。

ただし、何を「回せるもの」と見なすか、どうやって次の要素を取るかは、
言語側に全部ハードコードされているわけではありません。
この部分は `core::iter` の trait に委譲されています。

ここで大事なのは、これは「普通の外部ライブラリを勝手に呼ぶ」という話ではないことです。
`IntoIterator::into_iter` と `Iterator::next` は `core` にある特別な入り口で、
コンパイラはそれらを言語仕様の一部として使います。

つまり構造としては:

- 構文はコンパイラが知っている
- 反復の共通インターフェースは `core` の trait で表す
- 各型はその trait を実装して `for` に参加する

という分担です。

この設計にすると、コンパイラが `Vec` や slice だけを特別扱いしなくても、
自作型を `for` で回せるようにできます。

## `lang item` は普通の trait と何が違うか

普通の trait は、
「型がこのメソッドや関連型を持つ」という約束を表すだけです。
その trait を generic 制約に使ったり、メソッド呼び出しに使ったりできます。

一方 `lang item` は、
コンパイラが「この trait やこのメソッドを、言語機能の意味付けに使う」と
知っている特別な入り口です。

たとえば `for` では:

- `IntoIterator::into_iter`
- `Iterator::next`

が使われます。

つまり違いは「trait の形」よりも
「コンパイラがその trait を特別扱いするかどうか」にあります。

同じ名前の trait を自分で作っても、`for` はそれを見ません。
`for` が見るのは `core` にある、`lang item` として印の付いた側です。

同じ構造は他にもあります。

- `+` は `Add`
- `[]` は `Index`
- スコープ終端の破棄は `Drop`
- サイズ既知制約は `Sized`

に結びついています。

## `for x in xs` と `for x in &xs` で何が変わるか

変わるのは `expr` の型です。

- `for x in xs`
  `expr` の型は `Xs`
- `for x in &xs`
  `expr` の型は `&Xs`
- `for x in &mut xs`
  `expr` の型は `&mut Xs`

Rust はその型に対して使える `IntoIterator` 実装を trait 解決で選びます。

たとえば `Vec<T>` には概念的に次の 3 つがあります。

- `IntoIterator for Vec<T>` -> `Item = T`
- `IntoIterator for &Vec<T>` -> `Item = &T`
- `IntoIterator for &mut Vec<T>` -> `Item = &mut T`

だから:

- `for x in xs` では要素を値 `T` として取り出す
- `for x in &xs` では要素を参照 `&T` として取り出す
- `for x in &mut xs` では要素を可変参照 `&mut T` として取り出す

## `for x in xs.iter()` がなぜ動くのか

ここで `expr` は `xs.iter()` の結果です。
`Vec<T>` や slice の `iter()` は、要素を順に返すイテレータ型を返します。

つまり:

```rust
for x in xs.iter() {
    body
}
```

では `expr` はすでにイテレータです。

Rust では、`Iterator` を実装している型は `IntoIterator` としても使えます。
そのため `for` は
「`iter()` を呼ぶ」のではなく
「もう渡されてきたイテレータをそのまま回す」
と考えると正確です。

感覚的には次に近いです。

```rust
let mut it = xs.iter();

while let Some(x) = it.next() {
    body
}
```

## 自作型では何を実装すべきか

`for` で回せるようにする最低条件は `IntoIterator` です。

たとえば自作コレクション `Bag` で:

- `for x in bag`
- `for x in &bag`
- `for x in &mut bag`

を全部許したいなら、それぞれに対応する `IntoIterator` を用意します。

```rust
struct Bag(Vec<i32>);

impl IntoIterator for Bag {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Bag {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Bag {
    type Item = &'a mut i32;
    type IntoIter = std::slice::IterMut<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
```

これで `for` 側は何も特別なことをしなくても動きます。

## `iter()` と `iter_mut()` は必須か

必須ではありません。

`for` に必要なのは `IntoIterator` です。
ただし API としては:

- `iter()` で読み取りイテレータを返す
- `iter_mut()` で可変イテレータを返す
- `into_iter()` で消費イテレータを返す

という形をそろえるのが標準ライブラリの流儀です。

利用者にとっても分かりやすいので、自作コレクションでもこの形に寄せることが多いです。

## C と比べると

C の `for (i = 0; i < n; i++)` は
条件、更新、添字管理を自分で書くループです。

Rust の `for` はむしろ
「次の要素を返すオブジェクトに対して `next()` を呼び続ける」
という約束の上に乗っています。

つまり主役は `for` ではなく、`IntoIterator` と `Iterator` です。

## 今の段階での理解のゴール

- `for` が最初に呼ぶのは `IntoIterator::into_iter(expr)`
- `for x in xs` と `for x in &xs` の違いは、別の `IntoIterator` 実装が選ばれるから
- `for x in xs.iter()` は、`iter()` の戻り値そのものが回されている
- 自作型で `for` を使いたければ、必要な形に対して `IntoIterator` を実装する
- `iter()` と `iter_mut()` は必須ではないが、実用上はあると自然
