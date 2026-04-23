# データ型、パターン、エラー処理

このノートは、`struct`、`enum`、`match`、パターン、`Option<T>`、`Result<T, E>`、`unwrap` / `expect`、属性と `derive` をまとめます。

## `struct`

Rust の `struct` は、まず C の `struct` に近いです。

```rust
struct Point {
    x: i32,
    y: i32,
}
```

大きな違いは、`impl` ブロックでその型に関数をまとめられることです。

```rust
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
```

## 関連関数とメソッド

`self` 引数がないものは関連関数です。

```rust
let p = Point::new(1, 2);
```

`self` を受け取るものはメソッドです。

```rust
let n = p.norm1();
```

`p.norm1()` は感覚的には次に近いです。

```rust
Point::norm1(&p)
```

## `self` と `Self`

- `self`: メソッドが受け取る値そのもの、またはその参照
- `Self`: `impl` 対象の型名の別名

```rust
fn new(x: i32, y: i32) -> Self
```

これは `-> Point` と同じ意味です。

## `&self`, `&mut self`, `self`

`&self` は読み取り専用です。

```rust
fn norm1(&self) -> i32 {
    self.x.abs() + self.y.abs()
}
```

C の `const struct Point *self` に近いです。

`&mut self` は中身を書き換えます。

```rust
fn translate(&mut self, dx: i32, dy: i32) {
    self.x += dx;
    self.y += dy;
}
```

C の `struct Point *self` に近いですが、Rust では同時 alias まで検査されます。

`self` は所有権を受け取ります。

```rust
fn into_tuple(self) -> (i32, i32) {
    (self.x, self.y)
}
```

呼び出し後、元の値は基本的に使えません。値を分解したり別の型へ変換したりするときに使います。

## struct の便利な書き方

フィールド初期化省略記法:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
    }
}
```

構造体更新記法:

```rust
let user2 = User {
    email: String::from("new@example.com"),
    ..user1
};
```

`String` のような `Copy` でないフィールドが移動すると、元の構造体全体を使えなくなる場合があります。

tuple struct:

```rust
struct Color(i32, i32, i32);
```

unit-like struct:

```rust
struct AlwaysEqual;
```

フィールドはないが、型として区別したい、後で trait を実装したい、という場面で使えます。

## 属性と `derive`

`#[...]` は属性です。型や関数にメタ情報を付けます。

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

`derive` は「この trait の実装を自動生成してほしい」という指定です。

`Debug` を derive すると `{:?}` で表示できます。

```rust
let p = Point { x: 1, y: 2 };
println!("{p:?}");
println!("{p:#?}");
```

`dbg!(expr)` は式の所有権を受け取り、ファイル名、行番号、値を stderr に出し、その値を返します。

```rust
let p = dbg!(Point { x: 1, y: 2 });
```

`Debug`, `Clone`, `Copy` は derive しやすい代表例です。一方、`Add` のように型ごとの振る舞いを決める trait は普通は手で `impl` します。

## `enum`

Rust の `enum` は C の `enum + union` を安全にしたようなものです。

```rust
enum Value {
    Int(i32),
    Float(f32),
}
```

variant ごとに違う型・個数のデータを直接持てます。

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

enum にも `impl` でメソッドを定義できます。

`Vec<Enum>` が可能なのは、variant が違っても全体としては同じ 1 つの型だからです。

```rust
enum Value {
    Int(i32),
    Text(String),
}

let values: Vec<Value> = vec![
    Value::Int(10),
    Value::Text(String::from("hello")),
];
```

`Value::Int(10)` と `Value::Text(...)` は別々の型ではなく、どちらも `Value` 型の値です。
`Vec<T>` は「同じ `T` を連続して持つ」ので、`Vec<Value>` として格納できます。

メモリ上は、Rust の enum は安全な tagged union に近く、どの variant かを示す tag と、variant の中身を置く領域を持ちます。
`Value` 型全体のサイズは、基本的に一番大きい variant を収められる大きさに tag などを加えたものになります。
そのため `Vec<Value>` の各要素は同じサイズで連続配置できます。

## `match`

```rust
fn print_value(v: &Value) {
    match v {
        Value::Int(i) => println!("int: {i}"),
        Value::Float(f) => println!("float: {f}"),
    }
}
```

`match` は網羅的でなければなりません。処理漏れをコンパイラが止めます。

```rust
match n {
    0 => println!("zero"),
    1 | 2 => println!("small"),
    3..=9 => println!("medium"),
    _ => println!("large"),
}
```

`_` は値を使わない catch-all pattern です。

## パターン

パターンは「値の形を照合しながら、中身に名前を付ける書き方」です。

```rust
let pair = (10, 20);
let (x, y) = pair;
```

`Some(v)` や `Ok(text)` もパターンです。

```rust
match maybe {
    Some(v) => println!("{v}"),
    None => println!("none"),
}
```

パターンが出る場所:

- `match`
- `if let`
- `while let`
- `for`
- `let`
- 関数引数

## `if let` と `while let`

`if let` は、1 パターンだけを処理したいときの短縮形です。

```rust
if let Some(x) = value {
    println!("{x}");
}
```

必要なら `else` も付けられます。

`while let` は、パターンに合う間だけ繰り返します。

```rust
while let Some(x) = stack.pop() {
    println!("{x}");
}
```

これは概念的には次のような `match` です。

```rust
loop {
    match stack.pop() {
        Some(x) => println!("{x}"),
        None => break,
    }
}
```

## refutable と irrefutable

`let` の左辺には、必ず一致する irrefutable pattern が必要です。

```rust
let (x, y) = (1, 2);
```

`if let` や `while let` は、失敗しうる refutable pattern を受け付けます。

```rust
if let Some(x) = maybe {
    println!("{x}");
}
```

## pattern の細かい道具

複数 pattern:

```rust
match n {
    1 | 2 => println!("one or two"),
    _ => {}
}
```

範囲 pattern:

```rust
match n {
    1..=5 => println!("small"),
    _ => {}
}
```

`..` は残りを無視します。

```rust
let Point { x, .. } = p;
```

match guard:

```rust
match value {
    Some(x) if x % 2 == 0 => println!("even"),
    Some(_) => println!("odd"),
    None => println!("none"),
}
```

`@` binding は、パターンに一致させつつ値全体にも名前を付けます。

```rust
match n {
    id @ 3..=7 => println!("id: {id}"),
    _ => {}
}
```

## `Option<T>`

C の `NULL` 相当の「ないかもしれない」を、Rust では `Option<T>` で表します。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
fn find_even(xs: &[i32]) -> Option<i32> {
    for x in xs {
        if x % 2 == 0 {
            return Some(*x);
        }
    }
    None
}
```

呼び出し側は `None` を考慮する必要があります。

```rust
match find_even(&[1, 3, 8]) {
    Some(v) => println!("found: {v}"),
    None => println!("not found"),
}
```

## `Result<T, E>`

失敗しうる処理は `Result<T, E>` で表します。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `Ok(T)`: 成功した。値は `T`
- `Err(E)`: 失敗した。エラー情報は `E`

例:

```rust
use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
```

C の「戻り値で成功/失敗、詳細は `errno`」に近い役割ですが、成功値と失敗値の型が明示されます。

## `?`

`?` は `Result` のエラーを呼び出し元へ返す省略記法です。

```rust
fn main() -> Result<(), std::io::Error> {
    let text = read_file("hello.txt")?;
    println!("{text}");
    Ok(())
}
```

これは概念的には次です。

```rust
let text = match read_file("hello.txt") {
    Ok(text) => text,
    Err(err) => return Err(err),
};
```

`?` は `Err` を返すとき、必要に応じて `From` によるエラー型変換も使います。

`main() -> Result<(), E>` は、「プログラムが成功したか失敗したか」を返す形です。成功時に返したい値がないので `Ok(())` を返します。

## `unwrap()` と `expect()`

`unwrap()` は「中身があるはず」と決め打ちで取り出します。前提が外れると panic します。

```rust
let n: i32 = "123".parse().unwrap();
```

`expect()` も同じですが、失敗時メッセージを書けます。

```rust
let port: u16 = "8080"
    .parse()
    .expect("hard-coded port literal must be valid");
```

C で言えば、`NULL` ではない前提で強引に dereference する、または `assert(ptr != NULL)` に近い強さです。

## `unwrap()` を使いやすい場面

- テストコード
- 例示用の短いサンプル
- 固定文字列の `parse()` など、失敗したら実装ミスと言える場面
- 初期化時に必須データが壊れていたら続行不能な場面

避けたい場面:

- ユーザー入力
- ファイル I/O
- ネットワーク
- ライブラリの公開 API
- 長く動くサーバーやツール本体

そういう場所では `match` や `?` で失敗を扱う方が自然です。

## `unwrap_or()` と `unwrap_or_else()`

どちらも失敗時の代替値を返します。

- `unwrap_or(x)`: `x` を先に評価する
- `unwrap_or_else(f)`: 必要なときだけ `f` を呼ぶ

```rust
let x = maybe.unwrap_or(20);
let y = maybe.unwrap_or_else(|| expensive_default());
```

`Result` の `unwrap_or_else` では、`Err(e)` の `e` を受け取れます。

```rust
let v = result.unwrap_or_else(|e| {
    eprintln!("error: {e}");
    0
});
```

## panic

`panic!` は回復不能エラーを表します。デフォルトでは stack unwind して drop を走らせます。

`Cargo.toml` の profile で `panic = 'abort'` にすると unwind せず即終了し、バイナリを小さくできる場合があります。

backtrace を見たいとき:

```bash
RUST_BACKTRACE=1 cargo run
```

ユーザー入力や I/O の失敗は普通に起こるので、panic より `Result` で返すのが自然です。
