# Rust入門: C経験者向け

この文書は、C、アセンブリ、Linux、スタック/ヒープの基礎をすでに理解している人向けのRust入門です。
Rustは「低レベルに近い感覚を保ちつつ、Cで起きやすいメモリ破壊やデータ競合をコンパイル時に潰す」言語です。

## 1. Rustを一言で言うと

Cとの大きな違いは次の3つです。

1. デフォルトで安全
2. GCなしでメモリ管理する
3. 所有権と借用を型システムに組み込んでいる

Cでは、次のような問題が実行時まで残ります。

- 解放後アクセス
- 二重`free`
- NULL参照
- バッファオーバーラン
- 共有可変データの競合

Rustでは、これらのかなりの部分をコンパイル時に止めます。
代わりに最初はコンパイラがかなり細かく口を出してきます。
この「うるささ」がRustの本質です。

## 2. 最初の見え方: `rustc` と `cargo`

Cで言うと:

- `gcc`: コンパイラ
- `make`: ビルド管理

Rustで言うと:

- `rustc`: 単体コンパイル
- `cargo`: ビルド、依存管理、テスト、実行

最初は`cargo`中心で考えるのが実用的です。

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

`src/main.rs`:

```rust
fn main() {
    println!("hello, rust");
}
```

単体ファイルなら`rustc main.rs`でも動きますが、普段は`cargo`を使います。

よく使うコマンド:

- `cargo run`: ビルドして実行
- `cargo check`: 実行ファイルを作らず型チェック中心で高速確認
- `cargo test`: テスト実行
- `cargo build`: ビルド
- `cargo build --release`: 最適化ビルド

`cargo check`は、Cで言うと「リンクや生成物よりも先に、型と所有権の整合性を高速に検査する」道具として便利です。

## 3. 変数: デフォルトで不変

C:

```c
int x = 10;
x = 20;
```

Rust:

```rust
fn main() {
    let x = 10;
    // x = 20; // エラー

    let mut y = 10;
    y = 20;
    println!("{y}");
}
```

Rustでは、変数はデフォルトで不変です。
これは「書き換え可能性を明示する」設計です。

Cだと「とりあえず全部可変」になりがちですが、Rustでは逆です。
可変状態を減らすと、バグの可能性も減ります。

### シャドーイング

```rust
fn main() {
    let x = 10;
    let x = x + 1;
    let x = x * 2;
    println!("{x}");
}
```

同じ名前で再定義できます。これは再代入ではなく、新しい変数です。
型を変えても構いません。

## 4. 型: Cより推論が強いが、曖昧なら書く

```rust
fn main() {
    let a = 42;      // i32 と推論されやすい
    let b: u64 = 42; // 明示
    let c = 3.14;    // f64
    let d = true;    // bool
    let e = 'A';     // char
}
```

Rustの`char`はCの`char`とは違い、1バイトではなくUnicodeスカラー値です。
ASCII 1文字だけの型だと思わない方がよいです。

## 5. スタックとヒープ: Rustでも考え方は同じ

あなたが知っている理解はそのまま使えます。

- 固定サイズの小さな値は主にスタック
- 可変長データや動的確保はヒープ

例えば:

- `i32`, `bool`, `char`, 配列の一部はスタック寄り
- `String`, `Vec<T>`, `Box<T>` はヒープを使うことが多い

ただし重要なのは、Rustでは「ヒープ上のデータに誰が責任を持つか」が型に埋め込まれていることです。

## 6. 所有権: Rustの中心

Rustでは、値には基本的に「所有者」が1人います。
所有者がスコープを抜けると、値は破棄されます。

### Cでの感覚

```c
char *s = malloc(100);
strcpy(s, "hello");
free(s);
```

Cでは、`malloc`したメモリを誰が`free`するかは人間の責任です。

### Rustでの感覚

```rust
fn main() {
    let s = String::from("hello");
    println!("{s}");
} // ここで自動的に破棄される
```

これはGCではありません。
スコープ終端で確定的に破棄されます。C++のRAIIに近いです。

### ムーブ

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");
    // println!("{s1}"); // エラー: 所有権は移動済み
}
```

ここで起きていること:

- `String`本体の管理情報はスタックにある
- 実データはヒープにある
- `s2 = s1` でヒープの所有権が `s1` から `s2` に移る
- 二重解放を防ぐため、`s1` は以後使えない

Cの代入感覚で見ると不自然ですが、これが安全性の核心です。

### `Copy`型

```rust
fn main() {
    let x = 10;
    let y = x;
    println!("{x} {y}");
}
```

整数のような小さい値はコピーされます。
これはCの値渡しに近いです。

ざっくり:

- `i32`, `u64`, `bool` などは `Copy`
- `String`, `Vec<T>` などは通常 `Copy` ではない

## 7. 借用: 所有権を渡さずに使う

毎回ムーブすると不便なので、参照を使います。

```rust
fn print_len(s: &String) {
    println!("{}", s.len());
}

fn main() {
    let s = String::from("hello");
    print_len(&s);
    println!("{s}");
}
```

`&String` は参照です。所有権は移動しません。

Cで言うとポインタに見えますが、意味はかなり違います。

- Cのポインタ: 何でもできる
- Rustの参照: 有効期間と可変性が厳密に制約される

### 不変参照と可変参照

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");

    let r3 = &mut s;
    r3.push_str(" world");
    println!("{r3}");
}
```

Rustの重要ルール:

- 不変参照は同時に複数OK
- 可変参照は同時に1つだけ
- 不変参照と可変参照の同時存在は原則NG

これは単なる文法ではなく、データ競合防止のための規則です。

Cでは別名参照がいくらでも作れてしまい、
「読む側」と「書く側」が同時に存在して破綻しやすいです。
Rustはそこを型で止めます。

## 8. スライス: 配列や文字列の一部を見る

### 配列スライス

```rust
fn sum(xs: &[i32]) -> i32 {
    let mut total = 0;
    for x in xs {
        total += *x;
    }
    total
}

fn main() {
    let a = [1, 2, 3, 4];
    println!("{}", sum(&a));
    println!("{}", sum(&a[1..3]));
}
```

`&[i32]` は「`i32` の連続領域への借用」です。
感覚としては「ポインタ + 長さ」に近いです。

Cでありがちな:

```c
int sum(const int *xs, size_t len);
```

にかなり近いですが、Rust側は長さを型と値で自然に持てます。

### `String` と `&str`

Rustで最初に混乱しやすい点です。

- `String`: 所有する伸縮可能な文字列
- `&str`: 文字列スライス。所有しない参照

```rust
fn greet(name: &str) {
    println!("hello, {name}");
}

fn main() {
    let s = String::from("rust");
    greet(&s);
    greet("world");
}
```

`&str` を基本インターフェースにすると使いやすいです。

Cの `char *` との違い:

- Rust文字列はUTF-8
- `&str` は長さ情報を持つ
- NUL終端前提ではない

つまり、C文字列よりかなり安全ですが、その分「1文字 = 1バイト」という前提は通用しません。

## 9. 関数: 値渡し、参照渡し、戻り値

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x = add(2, 3);
    println!("{x}");
}
```

Rustは式ベースの言語です。
関数の最後の式は、`;` を付けなければ戻り値になります。

```rust
fn abs_diff(a: i32, b: i32) -> i32 {
    if a > b { a - b } else { b - a }
}
```

## 10. `struct`: Cの`struct`に近いが、`impl`でメソッドを持てる

Rustの `struct` 自体は、まずはCの `struct` とかなり近いです。
大きな違いは、その型に対する関数を `impl` ブロックにまとめて書けることです。

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn norm1(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn into_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn main() {
    let mut p = Point::new(3, -4);

    println!("{}", p.norm1());

    p.translate(10, 20);

    let t = p.into_tuple();
    println!("{t:?}");
}
```

ここで重要なのは、`impl Point` の中にある関数が2種類あることです。
    546 +### よくある誤解

- `Point::new(...)` のように `self` を受け取らないもの: 関連関数
- `p.norm1()` のように `self` を受け取るもの: メソッド

### `self` は「今そのメソッドを呼んでいる値」

最初は `self` を「C++ や Java の `this` に近いもの」と思ってよいです。
ただしRustでは、これは隠れた魔法というより「特殊な書き方ができる第1引数」です。

例えば:

```rust
p.norm1()
```

は感覚的には次とほぼ同じです。

```rust
Point::norm1(&p)
```

同様に:

```rust
p.translate(10, 20)
```

はだいたい次と同じです。

```rust
Point::translate(&mut p, 10, 20)
```

つまり `self` は、
「このメソッドがどの値を対象にして動くのか」を表しています。

### Cで書くとどう見えるか

同じ考え方をC寄りに書くとこうです。

```c
struct Point {
    int x;
    int y;
};

int point_norm1(const struct Point *self) {
    return abs(self->x) + abs(self->y);
}

void point_translate(struct Point *self, int dx, int dy) {
    self->x += dx;
    self->y += dy;
}
```

Rustのメソッドは、かなりこの形に近いです。
違うのは、「読み取り専用なのか」「書き換えるのか」「所有権を受け取るのか」を
`self` の型で明示する点です。

### `self` と `Self` は別物

ここはかなり重要です。

- `self`: そのメソッドが受け取る「値そのもの」
- `Self`: その `impl` の対象になっている「型そのもの」

この例では:

- `self` は `Point` の値、またはその参照
- `Self` は `Point` 型そのもの

なので:

```rust
fn new(x: i32, y: i32) -> Self
```

は

```rust
fn new(x: i32, y: i32) -> Point
```

と同じ意味です。

### `&self`: 読み取り専用

```rust
fn norm1(&self) -> i32 {
    self.x.abs() + self.y.abs()
}
```

`&self` は「`self` を不変参照で借りる」です。

意味:

- 所有権は受け取らない
- メソッド内で値を書き換えない
- 呼び出し後も元の値をそのまま使える

これはCの `const struct Point *self` にかなり近い感覚です。

### `&mut self`: 書き換える

```rust
fn translate(&mut self, dx: i32, dy: i32) {
    self.x += dx;
    self.y += dy;
}
```

`&mut self` は「`self` を可変参照で借りる」です。

意味:

- 所有権は受け取らない
- メソッド内で値を書き換えられる
- 呼び出し元でも同じ値が更新された状態で残る

このとき呼び出し側の変数も `mut` である必要があります。

```rust
let mut p = Point::new(1, 2);
p.translate(3, 4);
```

Cの `struct Point *self` に近いですが、
Rustでは「同時に他から安全でない形で触っていないこと」までコンパイラが見ます。

### `self`: 所有権を受け取る

```rust
fn into_tuple(self) -> (i32, i32) {
    (self.x, self.y)
}
```

これは参照ではなく、値そのものを受け取っています。

意味:

- メソッドが呼び出し元から所有権を受け取る
- 呼び出し後、元の変数は基本的に使えない
- 値を分解したり、別の型へ変換したり、消費したいときに使う

例:

```rust
let p = Point::new(3, 4);
let t = p.into_tuple();
// p はここではもう使えない
```

これはCにはあまりない感覚です。
Cでは値を渡すか、ポインタを渡すかを人間が管理しますが、
Rustでは「このメソッドは対象を消費する」と型で表現できます。

### どう使い分けるか

最初は次の基準で十分です。

- 読むだけなら `&self`
- 中身を書き換えるなら `&mut self`
- 呼び出し後に元の値を使わせたくない、あるいは分解して消費したいなら `self`

### よくある誤解

`self` は「クラスの中の特別な変数」というより、
「その型のメソッドが受け取る第1引数の省略記法」です。
なので、Rustのメソッドを理解するときは、
常に「これは所有権を取っているのか、借用しているのか」を見るのが重要です。

## 11. `enum`: Rustの強み

Cで状態を持つ値を表すとき、よくあるのは:

```c
typedef enum {
    TAG_INT,
    TAG_FLOAT
} Tag;

typedef struct {
    Tag tag;
    union {
        int i;
        float f;
    } data;
} Value;
```

Rustではこれをもっと安全に書けます。

```rust
enum Value {
    Int(i32),
    Float(f32),
}

fn print_value(v: &Value) {
    match v {
        Value::Int(i) => println!("int: {i}"),
        Value::Float(f) => println!("float: {f}"),
    }
}

fn main() {
    let a = Value::Int(42);
    let b = Value::Float(3.14);

    print_value(&a);
    print_value(&b);

    let values = vec![
        Value::Int(10),
        Value::Float(2.5),
        Value::Int(-7),
    ];

    for v in &values {
        print_value(v);
    }
}
```

ここでの使い方は次の通りです。

- `Value::Int(42)`: `Int` という種類の値を作る
- `Value::Float(3.14)`: `Float` という種類の値を作る
- `match v { ... }`: 種類ごとに分岐し、中に入っている値を取り出す

`print_value` が `&Value` を受け取っているのは、読むだけで所有権を奪いたくないからです。
もし `fn print_value(v: Value)` にすると、その関数を呼んだ時点で値はムーブされます。

`enum` は「タグ付きunion」を安全にしたようなものです。
しかも`match`で全パターンの処理を強制できます。

これはCにはない強い武器です。

## 12. `match`: 分岐の中心

```rust
fn describe(n: i32) {
    match n {
        0 => println!("zero"),
        1 | 2 => println!("small"),
        3..=9 => println!("medium"),
        _ => println!("large"),
    }
}
```

`switch`より強力で、網羅性チェックがあります。
「あり得るケースの処理漏れ」を減らせます。

## 13. `Option<T>`: NULLを型で表す

Cでは「値がない」を `NULL` で表すことが多いです。
Rustでは `Option<T>` を使います。

```rust
fn find_even(xs: &[i32]) -> Option<i32> {
    for x in xs {
        if x % 2 == 0 {
            return Some(*x);
        }
    }
    None
}

fn main() {
    let a = [1, 3, 5, 8];
    match find_even(&a) {
        Some(v) => println!("found: {v}"),
        None => println!("not found"),
    }
}
```

NULL参照の代わりに、
「この値は存在しない可能性がある」と型に明示されます。

## 14. `Result<T, E>`: エラー処理の基本

C:

- 戻り値で成功/失敗を返す
- 詳細は `errno`
- または出力引数に結果を書き込む

Rustでは、成功と失敗を次のような列挙型で表します。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### `<T>` や `<T, E>` は何か

ここでの `<T, E>` は「型の引数」です。
値ではなく、型を後から入れるための空欄だと思ってください。

- `T`: 成功時の値の型
- `E`: エラー時の型

なので `Result<T, E>` は、
「成功したときの型」と「失敗したときの型」をまだ決めていない、ひな型です。

例えば:

```rust
Result<i32, String>
```

と書いたら、

- 成功時は `i32`
- 失敗時は `String`

という意味になります。

感覚的には、頭の中でこう置き換えて構いません。

```rust
enum Result_i32_String {
    Ok(i32),
    Err(String),
}
```

同様に:

```rust
Result<(), std::io::Error>
```

は、

- 成功時は `()`
- 失敗時は `std::io::Error`

です。

`T` や `E` という名前は慣習にすぎません。
例えば次のように書いても意味は同じです。

```rust
enum Result<Success, Error> {
    Ok(Success),
    Err(Error),
}
```

### これはジェネリクス

この「型を後から入れられる仕組み」をジェネリクスと呼びます。

よく出るもの:

- `Option<T>`: ある型 `T` が「あるかもしれない、ないかもしれない」
- `Result<T, E>`: 成功時の型 `T` と失敗時の型 `E`
- `Vec<T>`: 要素の型が `T` の可変長配列

例えば:

```rust
Option<i32>
Option<String>
Vec<i32>
Vec<String>
```

は全部同じ「型のひな型」を使っています。

`Option<T>` を展開して見ると、こういう意味です。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

つまり:

- `Option<i32>` なら `Some(i32)` か `None`
- `Option<String>` なら `Some(String)` か `None`

です。

`Vec<T>` も同様で、

- `Vec<i32>` は `i32` の列
- `Vec<String>` は `String` の列

を表します。

### 関数にもジェネリクスがある

ジェネリクスは `enum` や `Vec<T>` のような型だけでなく、関数にも使えます。

```rust
fn identity<T>(x: T) -> T {
    x
}
```

この `<T>` は、
「この関数は、ある1つの型 `T` について動く」という意味です。

例えば:

```rust
fn main() {
    let a = identity(42);
    let b = identity("hello");

    println!("{a}");
    println!("{b}");
}
```

ここでは呼び出しごとに:

- `identity(42)` では `T = i32`
- `identity("hello")` では `T = &str`

と推論されます。

感覚的には、頭の中で次のように具体化されると思ってよいです。

```rust
fn identity_i32(x: i32) -> i32 {
    x
}

fn identity_str(x: &str) -> &str {
    x
}
```

もちろん実際にこういう関数を手で量産しているわけではありませんが、
理解としてはこれで十分です。

### 関数の `<T>` は何を表しているか

```rust
fn identity<T>(x: T) -> T
```

を分解すると:

- `fn`: 関数定義
- `identity`: 関数名
- `<T>`: 型引数を1つ受け取る
- `(x: T)`: 引数 `x` の型は `T`
- `-> T`: 戻り値の型も `T`

つまり「どんな型でもよいが、受け取った型と返す型は同じ」という関数です。

### 同じ `T` は同じ型を意味する

```rust
fn make_pair<T>(a: T, b: T) -> (T, T) {
    (a, b)
}
```

この場合、`a` と `b` は同じ型でなければなりません。

```rust
let ok = make_pair(10, 20);
// let ng = make_pair(10, "hello"); // エラー
```

後者がエラーなのは、1つの `T` に `i32` と `&str` を同時に入れられないからです。

もし別々の型を受け取りたいなら、型引数を2つにします。

```rust
fn make_pair2<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}
```

### 何でもできるわけではない

`T` は「まだ具体的に決まっていない型」なので、
その型に対して常に成り立つ操作しかできません。

例えばこれは動きます。

```rust
fn identity<T>(x: T) -> T {
    x
}
```

しかし、例えば `x + x` のような処理は、
「どんな `T` でも加算できる」とは限らないので、そのままでは書けません。
そういうときは後で学ぶ `trait` 制約が必要になります。

`trait` は、ざっくり言うと「その型に何ができるか」という約束です。
例えば:

- `std::fmt::Debug`: `{:?}` で表示できる
- `PartialOrd`: 比較できる
- `std::ops::Add`: `+` 演算ができる

`trait` 制約は次のように書きます。

```rust
fn print_debug<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
```

この

```rust
T: std::fmt::Debug
```

が「`T` は `Debug` を満たしていなければならない」という条件です。

例えば比較なら、こう書けます。

```rust
fn max_of_two<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

これは「`T` が比較可能な型なら動く」という意味です。

### なぜ `add_one<T>` はそのまま書けないのか

例えば次の関数を書きたくなるかもしれません。

```rust
fn add_one<T>(x: T) -> T {
    x + 1
}
```

しかし、これはそのままでは書けません。
理由は2つあります。

1. すべての `T` が `+` をサポートしているわけではない
2. `1` を `T` としてどう作るのかが決まっていない

例えば:

- `i32` なら `x + 1` は自然
- `String` に `+ 1` はできない
- `Point` のような自作型も、何もしなければ `+` は使えない

まずは「2つの同じ型を加算する」方が簡単です。

```rust
use std::ops::Add;

fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}
```

ここでの

```rust
T: Add<Output = T>
```

は、
「`T` 型どうしを `+` できて、その結果も `T` になる」
という条件です。

`add_one<T>` を本当に汎用に書こうとすると、
`+` できるだけでなく「`1` を `T` として作る方法」まで必要になります。
これは最初の段階では少し重いので、
まずは `Debug`、`PartialOrd`、`Add` くらいの制約に慣れるのがよいです。

### 自作型に `trait` を実装する最小例

ここまでで「`T: Add<Output = T>` のように書くのは分かったが、
自分の `struct` にどうやって `Add` を付けるのか」が気になるはずです。

最小例はこうです。

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

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 10, y: 20 };
    let p3 = p1 + p2;

    println!("{p3:?}");
}
```

この `impl Add for Point` は、
「`Point` 型に対して `+` をどう振る舞わせるか」を定義しています。

中身を分解すると:

- `impl Add for Point`: `Point` は `Add` を実装する
- `type Output = Point`: `p1 + p2` の結果の型は `Point`
- `fn add(self, rhs: Point) -> Point`: 左辺と右辺を受け取り、結果を返す

ここでの `self` は `+` の左辺です。
つまり:

```rust
p1 + p2
```

は感覚的には次に近いです。

```rust
Point::add(p1, p2)
```

この例では `add` が `self` と `rhs` を値で受け取っているので、
`p1` と `p2` はここで消費されます。
もし演算後も元の値を使いたいなら、`Copy` を付けるか、参照で設計する必要があります。

そしてこの `impl Add for Point` があるので、さきほどの

```rust
fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}
```

の `T` に `Point` を入れられるようになります。

### Cとの比較

Cで近いことをやろうとすると、

- `void *` を使う
- マクロでごまかす
- 型ごとに別関数を量産する

のどれかになりがちです。

Rustのジェネリクスは、そうした「いろいろな型で使い回したい」という要求を、
型安全を保ったまま表現する仕組みです。

意味:

- `Ok(T)`: 成功した。成功時の値は `T`
- `Err(E)`: 失敗した。失敗時の情報は `E`

例えば「ファイルを読んで文字列を返す」なら、成功時は `String`、失敗時は `std::io::Error` を返したいので、型はこうなります。

```rust
use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
```

これは次の意味です。

- 成功したら `Ok(String)`
- 失敗したら `Err(std::io::Error)`

例えば:

```rust
match read_file("hello.txt") {
    Ok(text) => println!("success: {text}"),
    Err(err) => println!("error: {err}"),
}
```

### `?` を使わない形

まずは、`Result` を手で展開する形を見ると分かりやすいです。

```rust
fn main() -> Result<(), std::io::Error> {
    let text = match read_file("hello.txt") {
        Ok(text) => text,
        Err(err) => return Err(err),
    };

    println!("{text}");
    Ok(())
}
```

ここで起きていること:

- `read_file("hello.txt")` は `Result<String, std::io::Error>` を返す
- `Ok(text)` なら中の `String` を取り出して `text` に代入する
- `Err(err)` なら、そのまま `main` から `Err(err)` を返して終了する
- 最後まで成功したら `Ok(())` を返す

### `?` を使う形

上のコードは `?` を使うと短く書けます。

```rust
use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn main() -> Result<(), std::io::Error> {
    let text = read_file("hello.txt")?;
    println!("{text}");
    Ok(())
}
```

`?` はだいたい次の省略記法です。

```rust
let text = match read_file("hello.txt") {
    Ok(text) => text,
    Err(err) => return Err(err),
};
```

つまり:

- `Ok(v)` なら中身の `v` を取り出す
- `Err(e)` なら `return Err(e)` する

Cで毎回 `if (ret < 0) return ret;` と書く感覚に近いですが、かなり読みやすいです。

### なぜ `main` が `Result` を返してよいのか

ここは引っかかりやすい点です。
`main` が返しているのは「ファイルの内容」ではなく、「このプログラムが成功したか失敗したか」です。

```rust
fn main() -> Result<(), std::io::Error>
```

の意味:

- 成功したら `Ok(())`
- 失敗したら `Err(std::io::Error)`

ここでの `()` は「成功時に特に返したい値はない」という意味です。

Rustの `main` は特別で、`Result<(), E>` を返せます。
ランタイムがそれを受け取って、

- `Ok(())` なら正常終了
- `Err(e)` ならエラー終了

として扱います。

感覚としてはCの次のコードに近いです。

```c
int main(void) {
    char *text = read_file("hello.txt");
    if (text == NULL) {
        return 1;
    }

    printf("%s\n", text);
    return 0;
}
```

対応づけると:

- `return 0;` に近いもの -> `Ok(())`
- `return 1;` に近いもの -> `Err(e)`

つまり、`main` が `Result` を返せるようにしておくと、途中で失敗した処理を `?` で自然に上へ返せます。

### `main` が `Result` を返さない書き方

もちろん、普通の `fn main()` にして自分で処理することもできます。

```rust
fn main() {
    match read_file("hello.txt") {
        Ok(text) => println!("{text}"),
        Err(err) => eprintln!("error: {err}"),
    }
}
```

ただしこの形では、`?` でそのままエラーを返せません。
そのため、簡単なCLIプログラムでは `main() -> Result<(), E>` の形がよく使われます。

## 15. `Vec<T>`: Rustの動的配列

```rust
fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    for x in &v {
        println!("{x}");
    }
}
```

感覚としては:

- Cの動的配列
- 長さと容量を持つ
- 再確保が起きることがある

ただし所有権と借用の規則があるので、安全に扱いやすいです。

## 16. `String`, `Vec`, `Box` のメモリ像

イメージとして:

```text
スタック:
    String / Vec / Box の管理情報

ヒープ:
    実データ
```

例えば `String` は内部的に「ポインタ、長さ、容量」に近い情報を持ちます。
ただし、それを直接いじるのではなく、安全なAPI経由で触ります。

重要なのは:

- どの値がヒープを所有しているか
- 参照はその所有者より長生きできない

これがダングリング参照を防ぎます。

## 17. ライフタイム: 参照の生存期間

Rust学習者が一度は詰まる点です。
ただし、最初から「注釈を書けるようになる」必要はありません。
まずは意味を理解すれば十分です。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` は「この戻り値は、入力参照のうち両方に共通して有効な期間の中でしか生きられない」という宣言です。

直感的には:

- C: 返したポインタが有効かは人間が気を付ける
- Rust: 有効期間をコンパイラに説明する

### ダングリング参照が止まる例

```rust
fn main() {
    let r: &str;

    {
        let s = String::from("hello");
        r = &s;
    }

    println!("{r}");
}
```

これはコンパイルできません。
内側のスコープを抜けると `s` は破棄されるので、その参照を外で使うのは危険だからです。

Cなら通ってしまい、未定義動作の種になります。

## 18. `mut` の意味はかなり重要

```rust
fn main() {
    let mut s = String::from("a");
    s.push('b');
}
```

`mut` は「その束縛を通じて値を変更できる」という意味です。
Rustでは可変性は明示されます。

Cでは `const` を付けない限り可変ですが、Rustは逆です。
この違いは思った以上に大きいです。

## 19. ループとイテレータ

Rustの `for` は、Cの

```c
for (i = 0; i < n; i++)
```

の形とは少し発想が違います。
Rustの `for` は「条件式を毎回評価するループ」というより、
「ある列から要素を1つずつ取り出す」構文です。

つまり:

```rust
for x in something {
    // ...
}
```

は、
`something` が生み出す要素を順番に `x` へ入れていく、という意味です。

```rust
fn main() {
    let v = vec![10, 20, 30];

    for x in &v {
        println!("{x}");
    }

    println!("{v:?}");
}
```

この例では `x` の型は `&i32` です。
`&v` と書いているので、`Vec<i32>` を借用し、要素を参照で1つずつ見ています。
そのため、ループ後も `v` は使えます。

### `for x in &v`: 読むだけ

```rust
fn main() {
    let v = vec![10, 20, 30];
    let mut sum = 0;

    for x in &v {
        sum += *x;
    }

    println!("sum = {sum}");
    println!("{v:?}");
}
```

ここでは:

- `x` の型は `&i32`
- `*x` で中身を読む
- `v` 自体の所有権は動かない

これは感覚的には `v.iter()` に近いです。

### `for x in &mut v`: 各要素を書き換える

```rust
fn main() {
    let mut v = vec![10, 20, 30];

    for x in &mut v {
        *x *= 2;
    }

    println!("{v:?}");
}
```

ここでは:

- `x` の型は `&mut i32`
- `*x = ...` や `*x *= 2` で要素を書き換えられる
- `v` を可変借用しているので、ループ中は他から同時に触れない

これは `v.iter_mut()` に近いです。

### `for x in v`: 要素を所有権ごと取り出す

```rust
fn main() {
    let v = vec![10, 20, 30];

    for x in v {
        println!("{x}");
    }

    // println!("{v:?}"); // エラー: v はムーブ済み
}
```

ここでは:

- `x` の型は `i32`
- 各要素を値として取り出す
- `v` 自体の所有権がループへ渡るので、ループ後に `v` は使えない

これは `v.into_iter()` に近いです。

`&v`, `&mut v`, `v` の違いは、Rustの所有権と借用そのものです。
`for` でもその差がそのまま出ます。

### 添字が必要なとき

最初はCの `for (i = 0; i < n; i++)` 感覚でも書けます。
例えば:

```rust
fn main() {
    let v = vec![10, 20, 30];

    for i in 0..v.len() {
        println!("{} {}", i, v[i]);
    }
}
```

ここでの `0..v.len()` は、
`0` 以上 `v.len()` 未満の値を順に作る範囲です。
つまり `0, 1, 2` が出ます。

`0..=v.len()` ではなく `0..v.len()` なのは、
最後の添字 `v.len()` は範囲外だからです。

### 添字が欲しいなら `enumerate()` が自然

要素も添字も欲しいとき、Rustでは次の形がよく使われます。

```rust
fn main() {
    let v = vec![10, 20, 30];

    for (i, x) in v.iter().enumerate() {
        println!("{} {}", i, x);
    }
}
```

このとき:

- `i` の型は `usize`
- `x` の型は `&i32`

です。

添字アクセス自体は悪くありませんが、
単に全要素を順番に見るだけなら `for x in &v` の方が自然です。

### `for` と `while` の違い

ここもC経験者には重要です。

- `for`: ある列や範囲から値を順番に取り出す
- `while`: 条件が真である間だけ繰り返す

Rustでは「回す対象が最初からある」なら、まず `for` を考えるのが自然です。

例えば 0 から 4 まで表示するだけなら、`for` の方が意図が直接出ます。

```rust
fn main() {
    for i in 0..5 {
        println!("{i}");
    }
}
```

同じことは `while` でも書けます。

```rust
fn main() {
    let mut i = 0;

    while i < 5 {
        println!("{i}");
        i += 1;
    }
}
```

ただし `while` では:

- ループ変数 `i` を自分で用意する
- 条件 `i < 5` を自分で管理する
- 更新 `i += 1` も自分で書く

必要があります。

Cではこれに慣れているので自然に感じますが、
Rustでは「範囲を順にたどるだけ」なら `for i in 0..5` の方が簡潔でミスが少ないです。

### `while` が向いている場面

`while` は、あらかじめ要素列があるというより、
「ある条件が変化していく間まわしたい」ときに向いています。

例えば:

```rust
fn main() {
    let mut n = 100;

    while n > 1 {
        println!("{n}");
        n /= 2;
    }
}
```

この例は、
「`n` が 1 より大きい間、半分にし続ける」
という条件ベースのループなので、`for` より `while` が自然です。

### 配列や `Vec` を回すなら、まず `for`

例えば `Vec` の全要素を表示したいだけなら、
次のような `while` はRustではやや低レベル寄りです。

```rust
fn main() {
    let v = vec![10, 20, 30];
    let mut i = 0;

    while i < v.len() {
        println!("{}", v[i]);
        i += 1;
    }
}
```

これは動きますが、

- 添字を自分で管理する
- 範囲外アクセスを避ける意識が必要
- 単純走査なのにコード量が増える

ので、普通は次の方が自然です。

```rust
fn main() {
    let v = vec![10, 20, 30];

    for x in &v {
        println!("{x}");
    }
}
```

### 使い分けの基準

- 連続した要素や範囲を順番にたどる -> `for`
- 条件が成り立つ間だけ繰り返す -> `while`
- 添字や状態を自分で細かく管理したい -> `while` も候補

要点を一枚でまとめると:

- `for x in &v` -> 読む。`x` は `&T`
- `for x in &mut v` -> 書き換える。`x` は `&mut T`
- `for x in v` -> 消費する。`x` は `T`
- 添字が欲しいときは `v.iter().enumerate()`
- 単純な走査なら `while` より `for` が自然

## 20. `unsafe`: Cに近い領域

Rustにも `unsafe` があります。

```rust
unsafe {
    // 生ポインタ逆参照など
}
```

ここではコンパイラが安全性を全面保証しません。
つまり、`unsafe` の中はCに近い責任を負います。

用途は主に:

- FFI
- OSやハードウェアに近い処理
- 安全抽象化の内部実装

重要なのは、`unsafe` が「言語全体」ではなく「狭い区画」に閉じ込められることです。

## 21. C経験者が最初につまずく点

### 1. 代入しただけで元が使えなくなる

`String` や `Vec` はムーブするからです。

対策:

- 参照 `&T` を使う
- 必要なら `clone()` する

### 2. 可変参照の制約が厳しい

エイリアスと変更を同時に許さないためです。
最初は窮屈でも、慣れるとかなり安全です。

### 3. `String` と `&str` の違いが曖昧

最初は次の感覚で十分です。

- 所有したい: `String`
- 借りて読むだけ: `&str`

### 4. NULLがない

`Option<T>` を使います。

### 5. エラー処理が戻り値ベース

`Result<T, E>` と `?` に慣れる必要があります。

## 22. 学び始めの実用ルール

最初は次の方針で十分です。

1. 関数の引数は、所有権が不要なら `&T` か `&str` を使う
2. 可変が必要なときだけ `mut` を付ける
3. 配列や`Vec`は、まず `for x in &v` で回す
4. エラーがあり得る処理は `Result` を返す
5. 「所有する文字列は `String`、借りる文字列は `&str`」を徹底する

## 23. 最初に覚えるべき標準ライブラリ

優先度順にこれです。

- `String`
- `Vec<T>`
- `Option<T>`
- `Result<T, E>`
- `std::fs`
- `std::collections::HashMap`

最初の段階では、外部クレートより標準ライブラリ中心で十分です。

## 24. Cとの対応を一枚で整理

- `struct` -> Rustの `struct`
- `enum + union` -> Rustの `enum`
- `char *` -> 文脈次第で `String`, `&str`, `Vec<u8>`
- `malloc/free` -> 所有権 + 自動破棄
- `NULL` -> `Option<T>`
- 返り値 + `errno` -> `Result<T, E>`
- `const T *` -> `&T` に少し近い
- `T *` -> `&mut T` や生ポインタ `*mut T` に分かれる

ただし、完全な1対1対応ではありません。
Rustは「どの操作が安全か」を型で細かく分けています。

## 25. 最初の学習順序

この順で進めると無理が少ないです。

1. `let`, `mut`, 基本型
2. `String`, `&str`, `Vec`
3. 所有権と借用
4. `struct` と `impl`
5. `enum`, `match`, `Option`, `Result`
6. スライス
7. ライフタイム
8. 必要になってから `trait`, `generics`, `unsafe`

## 26. 手を動かす課題

### 課題1: Cの`strlen`相当を書く

```rust
fn my_strlen(s: &str) -> usize {
    s.len()
}
```

まずは `&str` に慣れるのが目的です。

### 課題2: 整数配列の最大値を返す

戻り値は `Option<i32>` にしてください。
「見つからない可能性」を型で表す練習になります。

### 課題3: 可変長スタックを作る

`Vec<i32>` を内部に持つ `struct Stack` を作り、
`push`, `pop`, `peek` を実装してください。

`pop` と `peek` は `Option<i32>` や `Option<&i32>` を返すと良いです。

### 課題4: ファイルを読んで行数を数える

`Result<usize, std::io::Error>` を返してください。
`?` の練習になります。

### 課題5: Cのタグ付きunionをRustの`enum`へ移植する

これをやると `match` の強さが分かります。

## 27. 今の段階で無理に覚えなくてよいもの

最初から深追いしなくてよいです。

- 高度なライフタイム注釈
- `trait object`
- 非同期処理
- マクロ自作
- 高度な並行処理
- `unsafe` の細部

まずは「所有権と借用でメモリ安全をどう作るか」を体に入れる方が重要です。

## 28. まとめ

Rustは、Cの代わりに「何もかも自由に触れる」言語ではありません。
その代わり、

- 所有者は誰か
- 今は読むだけか、書くのか
- この参照はどこまで生きてよいか

をコンパイラが検査します。

C経験者にとって最重要なのは次の再解釈です。

- ポインタの自由度を減らし、安全性を型に移したものがRustの参照
- `malloc/free` の責任を、所有権規則に変えたものがRustのメモリ管理
- `NULL` やエラーコードを、列挙型で明示したものが `Option` と `Result`

最初は窮屈に見えますが、慣れると「未定義動作の可能性を頭で追い続ける負担」がかなり減ります。

---

次に読むとよいテーマ:

1. `trait` とジェネリクス
2. `Iterator`
3. `Rc`, `Arc`, `RefCell`, `Mutex`
4. FFI と `unsafe`

この文書を読んだら、次は短いプログラムを3つほど自分で書いて `cargo check` を回し、コンパイラのエラー文に慣れるのが最短です。
