# traits_basics

## Status

- Current status: learning in progress
- Priority: highest
- Goal: `trait` を「何の仕組みか」「どう読むか」「どこで使うか」で説明できるようにする

## First Answer

trait は、
「ある型がどんな振る舞いを持つか」を表す約束です。

最初は次の2つだけ覚えるとかなり整理しやすいです。

- `impl Trait for Type`
  - `Type` に `Trait` の振る舞いを与える
- `T: Trait`
  - 型 `T` は `Trait` を満たしていなければならない

この2つは別物ではなく、同じ trait を別方向から見ています。

## Trait は何か

一番実用的な理解はこれです。

- `struct` / `enum`: データの形
- `trait`: その型で何ができるか

なので、trait は「インターフェース」に近いです。
ただし、Rustでは「関数の一覧」だけではなく、
演算子、表示、比較、参照っぽい振る舞いまで trait で表します。

つまり trait は、
「ある型が持つ能力や振る舞いの約束」
と考えるのが自然です。

## 一番短い例

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("wan");
    }
}
```

ここでは:

- `Animal`: 「鳴ける」という約束
- `Dog`: 具体的な型
- `impl Animal for Dog`: `Dog` は `Animal` の約束を満たす

です。

## `impl Trait for Type` の読み方

これは最重要です。

```rust
impl Animal for Dog {
    fn speak(&self) {
        println!("wan");
    }
}
```

を日本語で読むと:

- `impl`: 実装する
- `Animal`: この trait を
- `for Dog`: `Dog` 型に対して

です。

つまり:

```rust
impl Animal for Dog
```

は

「Dog に Animal の振る舞いを実装する」

と読めます。

これが trait の基本形です。

## `T: Trait` は何か

今度は使う側です。

```rust
fn make_it_speak<T: Animal>(x: &T) {
    x.speak();
}
```

ここでの

```rust
T: Animal
```

は

「型 `T` は `Animal` を実装していなければならない」

という意味です。

つまり:

- `impl Animal for Dog`
  - `Dog` 側が「私は Animal です」と名乗る
- `T: Animal`
  - 関数側が「Animal な型だけ受け付けます」と要求する

です。

この2つがつながると:

```rust
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("wan");
    }
}

fn make_it_speak<T: Animal>(x: &T) {
    x.speak();
}

fn main() {
    let d = Dog;
    make_it_speak(&d);
}
```

が成り立ちます。

## trait は「後から機能を足す」仕組みか

かなりその見方で大丈夫です。

- `struct` / `enum` でデータの形を定義する
- `impl Type { ... }` でその型固有のメソッドを書く
- `impl Trait for Type { ... }` で共通ルールに沿った振る舞いを足す

例えば:

- `impl Point { fn norm1(&self) -> i32 { ... } }`
  - `Point` 固有の普通のメソッド
- `impl Add for Point { ... }`
  - `+` という共通ルールを `Point` に与える
- `impl Display for Point { ... }`
  - 表示という共通ルールを `Point` に与える

です。

## `impl Type { ... }` と `impl Trait for Type { ... }` の違い

ここも混ざりやすい点です。

### 普通の `impl`

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn norm1(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
```

これは

「Point 型そのものにメソッドを書く」

形です。

### trait 実装の `impl`

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

これは

「Display という共通ルールに沿って、Point の表示方法を書く」

形です。

## `Add` で見る

`+` は trait で動いています。

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

これは:

- `Add`: `+` のルール
- `Point`: そのルールを実装する型
- `add(self, rhs)`: 実際に何をするか

です。

なので

```rust
let p3 = p1 + p2;
```

が書けます。

感覚的には:

```rust
Point::add(p1, p2)
```

に近いです。

## `Display` で見る

表示も trait です。

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

これで:

```rust
println!("{p}");
```

が書けます。

ここで重要なのは、
`Add` と `Display` は全然違う用途なのに、両方とも trait だという点です。

共通しているのは:

- 「ある振る舞いのルール」が trait で表される
- 型ごとに `impl Trait for Type` で実装する

という構造です。

## `Deref` で見る

`*` や「参照っぽい振る舞い」にも trait が使われます。

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

これで:

```rust
let x = MyBox(42);
println!("{}", *x);
```

ができます。

つまり trait は、

- `+`
- `+=`
- `*`
- 表示
- 比較

のような「言語に組み込まれているように見える振る舞い」まで表します。

## `type Output` は何か

ここもつまずきやすい点です。

`Add` の定義はこうでした。

```rust
trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

`type Output;` は関連型です。

最初はこう理解すれば十分です。

- trait の中にある「型の空欄」
- 実装側が具体的な型を決めるための場所

例えば:

```rust
impl Add for Point {
    type Output = Point;
    ...
}
```

なら、

- 左辺は `Point`
- 右辺は `Point`
- 結果も `Point`

です。

つまり `type Output` は、
「この trait のこの実装では、結果型は何か」
を表しています。

## なぜ関連型が必要なのか

`Add` は「右辺の型」だけでなく「結果の型」も持ちたいからです。

例えば設計としては:

- `Point + Point -> Point`
- `Point + Vector -> Point`
- `Meters + Meters -> Meters`

のように、結果型は実装ごとに違い得ます。

そのため、trait の中に

```rust
type Output;
```

という空欄を置いて、
各 `impl` がそれを埋めます。

## `Rhs = Self` は何か

これも `Add` の中にあります。

```rust
trait Add<Rhs = Self> {
    ...
}
```

これは

「右辺型 `Rhs` は、省略したら `Self` と同じにする」

という意味です。

だから:

```rust
impl Add for Point
```

は、実質

```rust
impl Add<Point> for Point
```

です。

右辺を別型にしたければ:

```rust
impl Add<i32> for Point {
    type Output = Point;

    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
```

のように書けます。

## trait とジェネリクスのつながり

これも重要です。

```rust
fn add_two<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
```

ここでは:

- ジェネリクス: `T`
- trait 制約: `T: Add<Output = T>`

です。

つまり:

- ジェネリクスは「どんな型にも使えるようにする仕組み」
- trait 制約は「ただし、この能力は持っていてほしいという条件」

です。

これを日本語で読むと:

「`T` は何でもよいが、`+` が使えて、その結果も `T` になる型であってほしい」

です。

## ここまでの一枚絵

- trait = 振る舞いの約束
- `impl Trait for Type` = 型にその約束を実装する
- `T: Trait` = その約束を満たす型だけ受け付ける
- 関連型 = trait 内の型の空欄
- `Add`, `Display`, `Deref` は全部 trait の例

## Cとの比較

Cで近いことをやろうとすると、だいたい次のどれかです。

- 関数ポインタを持つ
- `void *` と手動キャストを使う
- 型ごとに別関数を大量に作る
- 演算子オーバーロードはできないので命名で頑張る

Rustの trait は、
それらを型安全にまとめる仕組みだと見ると理解しやすいです。

特に `Display` や `Add` のような trait は、
Cでは言語として統一的に扱いにくい部分を、共通ルールとしてまとめています。

## 今の段階での理解目標

まずはここまでで十分です。

1. trait は「振る舞いの約束」
2. `impl Trait for Type` は「その型にその約束を実装する」
3. `T: Trait` は「その約束を満たす型だけ受け取る」
4. `type Output` は「その trait 実装での結果型」
5. `Add`, `Display`, `Deref` は trait の具体例

## Next Practice

次に自分で書くと理解しやすいです。

1. `trait Animal` を作って `Dog` と `Cat` に実装する
2. `struct Point` に `Display` を実装する
3. `struct Point` に `Add` を実装する
4. `fn print_anything<T: std::fmt::Display>(x: T)` を書く
5. `MyBox<T>` に `Deref` を実装する

## Links

- [[Home]]
- [[knowledge/traits_generics_and_iterators]]
- [[daily/2026-04-03]]
