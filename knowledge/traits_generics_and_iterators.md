# trait、generics、iterator

このノートは、generics、trait、trait bound、`where`、関連型、trait object、`for` / `IntoIterator`、iterator / `Option` の `filter()` をまとめます。

## generics

Generics は「型を後から入れられる仕組み」です。

```rust
fn identity<T>(x: T) -> T {
    x
}
```

`<T>` は「この関数は、ある 1 つの型 `T` について動く」という意味です。

```rust
let a = identity(42);      // T = i32
let b = identity("hello"); // T = &str
```

同じ `T` は同じ型を意味します。

```rust
fn make_pair<T>(a: T, b: T) -> (T, T) {
    (a, b)
}

let ok = make_pair(10, 20);
// let ng = make_pair(10, "hello"); // 型が違うのでエラー
```

別々の型を受けたいなら、型引数を分けます。

```rust
fn make_pair2<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}
```

## 「別の変数と同じ型」

stable Rust には `typeof(expr)` のような構文はありません。

変数宣言では、まず型推論に任せるのが自然です。

```rust
let a = 10usize;
let b = a + 20;
```

`let b: _ = a + 20;` の `: _` は「型を推論して」の意味です。「`a` と同じ型」という専用構文ではありません。

同じ型名を何度も使うなら型エイリアスを使います。

```rust
type UserId = u64;

let a: UserId = 10;
let b: UserId = 20;
```

「この 2 つは同じ型でなければならない」という関係は generics で表します。

## trait

`trait` は「その型に何ができるか」の約束です。

```rust
trait Speak {
    fn speak(&self);
}
```

`struct` はデータの形、`trait` は操作の約束です。

```rust
struct Dog {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{}: wan", self.name);
    }
}
```

- `impl Dog { ... }`: `Dog` 固有のメソッド
- `impl Speak for Dog { ... }`: `Dog` が `Speak` の約束を満たす実装

## trait bound

```rust
fn make_it_speak<T: Speak>(x: &T) {
    x.speak();
}
```

`T: Speak` は trait bound です。

意味:

- `T` は何でもよいわけではない
- `Speak` を実装している型だけ受け付ける
- 関数の中では `Speak` が保証する操作だけ使える

これは実行時チェックではなく、コンパイル時の条件です。

## `where`

短い bound は型引数の横に書けます。

```rust
fn print_debug<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
```

長くなると `where` が読みやすいです。

```rust
use std::ops::Add;

fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}
```

`where` は「この型にはこういう条件が必要」と後ろにまとめる構文です。

## `Add<Output = T>` と関連型

`Add` trait は、加算結果の型を `Output` という関連型で持っています。

```rust
T: Add<Output = T>
```

これは「`T + T` ができて、その結果も `T`」という条件です。

自作型に `Add` を実装する例:

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

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

`type Output = Point;` が「`p1 + p2` の結果型は `Point`」という指定です。

## 関連型

関連型は trait 内で定義される「実装ごとに決まる型」です。

`Iterator` の `Item` が代表例です。

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

`Self::Item` は「この実装で決めた `Item` 型」という意味です。

`IntoIterator` では次のような関連型が出ます。

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

## `impl<'a>`

`impl<'a>` は、「この `impl` ブロックの中で lifetime 名 `'a` を使う」という宣言です。

```rust
impl<'a> IntoIterator for &'a Bag {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
```

`for &'a Bag` は「`Bag` そのもの」ではなく「`'a` だけ有効な `Bag` への参照」に対して trait を実装する、という意味です。

## `impl Trait`

引数で使う `impl Trait`:

```rust
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

これは次に近い短縮形です。

```rust
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

戻り値の `impl Trait` は、「ある 1 つの具体型を返すが、呼び出し側には trait として見せる」という意味です。

```rust
fn returns_summarizable() -> impl Summary {
    NewsArticle { /* ... */ }
}
```

分岐ごとに別の具体型を返す用途ではありません。その場合は enum や trait object を検討します。

## trait object

trait bound はコンパイル時に具体型が決まる使い方です。

trait object は、具体型を隠して trait の約束だけで扱う使い方です。

```rust
fn make_it_speak_dyn(x: &dyn Speak) {
    x.speak();
}
```

`&dyn Speak` は感覚的には次の 2 つを持ちます。

- 実データへのポインタ
- その型用の vtable へのポインタ

つまり `&dyn Speak` は wide pointer です。

```text
&dyn Speak = data_ptr + vtable_ptr
```

`&str` や `&[T]` も wide pointer です。

```text
&str      = data_ptr + len
&[T]      = data_ptr + len
&dyn Trait = data_ptr + vtable_ptr
```

`dyn Speak` そのものはサイズがコンパイル時に分からないので、普通は `&dyn Speak`, `Box<dyn Speak>`, `Rc<dyn Speak>`, `Arc<dyn Speak>` のように参照や所有ポインタ越しに扱います。

## `Box<dyn Trait>`

`Box<dyn Speak>` は、実データを heap に置き、stack 側には data pointer と vtable pointer を持ちます。

heap に置かれるのは具体型の値です。vtable まで heap に複製されるわけではありません。

`Vec<Box<dyn Draw>>` のようにすると、異なる具体型を同じ collection に入れられます。

## object safety

trait object にできる trait には制約があります。

典型的には、戻り値に `Self` を使う method や generic method は object safety に引っかかります。

最初は「`dyn Trait` にするには、実行時に vtable だけで呼べる形の method である必要がある」と捉えるとよいです。

## `Sized` と `?Sized`

Rust の通常の generic type parameter は、デフォルトで `Sized` を要求します。

```rust
fn f<T>(x: &T) {}
```

これは概念的には `T: Sized` があると思ってよいです。

`str`, `[T]`, `dyn Trait` のようなサイズ不定型も受けたいときは `?Sized` を使います。

```rust
fn f<T: ?Sized>(x: &T) {}
```

`?Sized` は「`Sized` でなくてもよい」という意味です。ただし、値そのものを直接受け取るのではなく、参照や pointer 越しに扱う必要があります。

## `for` と `IntoIterator`

Rust の `for` は `IntoIterator` を使います。

```rust
for x in expr {
    body
}
```

概念的には次です。

```rust
let mut it = IntoIterator::into_iter(expr);

while let Some(x) = it.next() {
    body
}
```

## `for x in v`, `&v`, `&mut v`

```rust
let v = vec![10, 20, 30];

for x in &v {
    println!("{x}");
}
println!("{v:?}");
```

`for x in &v` は読むだけです。`x` は `&T` です。ループ後も `v` は使えます。

```rust
let mut v = vec![10, 20, 30];

for x in &mut v {
    *x *= 2;
}
```

`for x in &mut v` は各要素を書き換えます。`x` は `&mut T` です。

```rust
let v = vec![10, 20, 30];

for x in v {
    println!("{x}");
}
// v はここでは使えない
```

`for x in v` は要素を所有権ごと取り出します。`x` は `T` です。

対応:

- `for x in &v`: `v.iter()` に近い
- `for x in &mut v`: `v.iter_mut()` に近い
- `for x in v`: `v.into_iter()` に近い

## 添字と `enumerate()`

添字が欲しいなら `enumerate()` が自然です。

```rust
let v = vec![10, 20, 30];

for (i, x) in v.iter().enumerate() {
    println!("{i}: {x}");
}
```

`i` は `usize`、`x` は `&i32` です。

単に全要素を見るだけなら、添字ループより `for x in &v` の方が自然です。

## iterator は lazy

iterator adaptor は、それだけでは実行されません。

```rust
let v = vec![1, 2, 3];
let it = v.iter().map(|x| x * 2); // まだ計算しない
let out: Vec<_> = it.collect();    // ここで消費する
```

- `map`, `filter`: iterator adaptor。別の iterator を返す
- `sum`, `collect`: consuming adaptor。iterator を消費する

最適化後には、iterator を使ったコードは手書き loop と同等の性能になることを狙って設計されています。

## `Iterator::filter()`

`Iterator::filter()` は複数要素の流れから、条件に合う要素だけを残します。

```rust
let xs = vec![1, 2, 3, 4];
let evens: Vec<_> = xs.into_iter().filter(|x| x % 2 == 0).collect();
```

`filter` の closure は要素への参照を受け取ることが多いので、型に注意します。

## `Option::filter()`

`Option::filter()` は、`Some(v)` のときだけ条件を見て、合わなければ `None` にします。

```rust
let x = Some(10).filter(|n| *n > 5);
assert_eq!(x, Some(10));

let y = Some(3).filter(|n| *n > 5);
assert_eq!(y, None);
```

`match` で書くと:

```rust
match opt {
    Some(v) if predicate(&v) => Some(v),
    _ => None,
}
```

## `split_once(...).filter(...)`

```rust
let parsed = line
    .split_once('=')
    .filter(|(key, _value)| !key.is_empty());
```

読み方:

1. `split_once('=')` は `Option<(&str, &str)>`
2. `Some((key, value))` なら `filter` の条件を見る
3. 条件が true ならそのまま `Some((key, value))`
4. false か、もともと `None` なら `None`

`Iterator::filter()` は複数要素の流れ、`Option::filter()` は 0 個か 1 個の値に対する条件チェックです。

## closure と `Fn` traits

closure は匿名関数で、周囲の変数を capture できます。

```rust
let factor = 2;
let f = |x| x * factor;
```

capture の仕方に応じて、closure は次の trait を実装します。

- `Fn`: 共有参照だけで呼べる
- `FnMut`: 可変に capture した状態を変更できる
- `FnOnce`: capture した値を消費できるので一度だけ呼べる

`move` closure は使う値の所有権を closure に移します。thread に渡す closure でよく使います。

```rust
let s = String::from("hello");
let f = move || println!("{s}");
```

## function pointer

関数ポインタ型は `fn(i32) -> i32` のように書きます。

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

let f: fn(i32) -> i32 = add_one;
```

function pointer は `Fn`, `FnMut`, `FnOnce` の 3 trait すべてを実装します。
