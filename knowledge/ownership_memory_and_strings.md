# 所有権、借用、メモリ、文字列

このノートは、所有権、借用、`Copy` / `Clone`、スライス、`String` / `&str`、UTF-8、`char_indices()`、lifetime の基礎をまとめます。

## スタックとヒープ

Rust でも C と同じく、スタックとヒープの基本的な考え方は使えます。

- 固定サイズの小さな値は主にスタック
- 可変長データや動的確保はヒープ

例:

- `i32`, `bool`, `char`, 固定長配列はスタック寄り
- `String`, `Vec<T>`, `Box<T>` はヒープを使うことが多い

重要なのは、Rust では「ヒープ上のデータに誰が責任を持つか」が型と所有権規則に埋め込まれていることです。

## 所有権の規則

Rust の値には基本的に所有者が 1 人います。

- 各値には所有者が 1 つある
- 所有者がスコープを抜けると値は破棄される
- 同時に所有者は 1 つだけ

```rust
fn main() {
    let s = String::from("hello");
    println!("{s}");
} // ここで s が drop される
```

これは GC ではありません。スコープ終端で確定的に破棄されます。C++ の RAII に近いです。

## ムーブ

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");
    // println!("{s1}"); // エラー
}
```

`String` はヒープバッファを所有します。`let s2 = s1;` では所有権が `s1` から `s2` に移ります。二重解放を避けるため、`s1` は以後使えません。

C の代入感覚では不自然ですが、「誰が free するか」を型で一意にする仕組みです。

## `clone()`

`clone()` は明示的な複製です。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("{s1} {s2}");
```

`String` の `clone()` はヒープデータまで複製します。C で言う shallow copy ではなく、所有するデータを新しく用意する操作です。

## `Copy` と `Clone`

整数のような小さい値は `Copy` です。

```rust
let x = 10;
let y = x;
println!("{x} {y}");
```

`Copy` 型は、代入や引数渡しで暗黙にビットコピーされ、元の変数も使えます。

代表例:

- 整数型
- 浮動小数点数
- `bool`
- `char`
- 要素がすべて `Copy` な tuple

`String` や `Vec<T>` は通常 `Copy` ではありません。ヒープ所有権を単純コピーすると二重解放につながるためです。

自作型でも、全フィールドが `Copy` なら `derive` できます。

```rust
#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

`Copy` 型は `Clone` でもあるべきなので、通常は `#[derive(Clone, Copy)]` と書きます。

## 借用

所有権を渡さずに値を使いたいときは参照を使います。

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

`&String` は所有権を移しません。C のポインタに見えますが、Rust の参照は有効期間と可変性が制約されます。

## 共有参照と可変参照

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

重要ルール:

- 共有参照 `&T` は同時に複数 OK
- 可変参照 `&mut T` は同時に 1 つだけ
- 共有参照と可変参照の同時存在は原則 NG

C では alias をいくらでも作れるので、読む側と書く側が同時に存在して壊れやすいです。Rust はそこを型で止めます。

## 配列 `[T; N]`

配列は「`T` を `N` 個ちょうど持つ値」です。

```rust
let a: [i32; 4] = [10, 20, 30, 40];
let b = [0; 8];
```

長さ `N` は型の一部です。`[i32; 4]` と `[i32; 5]` は別の型です。

配列添字が範囲外なら実行時 panic します。C の未定義動作とは違います。

## スライス `[T]` と `&[T]`

スライス `[T]` は、連続した `T` の領域そのものを表すサイズ不定型です。実際によく使うのは `&[T]` です。

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

`&[i32]` は「データ先頭ポインタ + 長さ」に近いです。

C なら:

```c
int sum(const int *xs, size_t len);
```

Rust では `&[i32]` が pointer と length をセットで持つため、長さの渡し忘れが起きにくくなります。

## `&[T]` と `&[T; N]`

```rust
let a = [10, 20, 30, 40];

let whole1 = &a;     // &[i32; 4]
let whole2 = &a[..]; // &[i32]
```

`&[i32; 4]` は「長さ 4 の配列への参照」です。基本はポインタ 1 個です。

`&[i32]` は「スライスへの参照」です。基本はデータポインタ + 長さです。

64bit 環境のよくあるサイズ感:

- `&[i32; 4]`: 8 bytes
- `&[i32]`: 16 bytes

ただし、これは参照値のサイズであり、元の配列本体のサイズではありません。

## API では `&[T]` が便利

```rust
fn sum(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

fn main() {
    let a = [1, 2, 3, 4];
    let v = vec![1, 2, 3, 4];

    println!("{}", sum(&a));
    println!("{}", sum(&v));
    println!("{}", sum(&v[1..3]));
}
```

`&[T]` を受け取る関数にしておくと、配列、`Vec<T>`、一部分の slice を同じ形で受け取れます。

## `String` と `&str`

最初に混乱しやすい組です。

- `String`: 所有する伸縮可能な UTF-8 文字列
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

関数が文字列を読むだけなら、引数は `&str` が自然です。`String` でも文字列リテラルでも呼べます。

C の `char *` との違い:

- Rust 文字列は UTF-8
- `&str` は長さ情報を持つ
- NUL 終端前提ではない

## `String` の基本操作

```rust
let mut s = String::new();
s.push_str("hello");
s.push('!');
```

作り方:

```rust
let a = String::new();
let b = "hello".to_string();
let c = String::from("hello");
```

結合:

```rust
let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = s1 + &s2; // s1 はムーブされる
```

複数の値を組み立てるなら `format!` が読みやすいことが多いです。

```rust
let name = "rust";
let msg = format!("hello, {name}");
```

## `String` は添字で 1 文字を取れない

Rust の文字列は UTF-8 です。1 文字が 1 byte とは限りません。

そのため、`s[0]` で「最初の文字」を取る、という操作はできません。

文字列を見る単位:

- bytes: UTF-8 の byte
- chars: Unicode scalar value
- grapheme cluster: 人間が見た 1 文字に近い単位。標準ライブラリだけでは扱いきれないことがある

## `char` と UTF-8 の byte 数

Rust の `char` 型そのものは常に 4 bytes です。

一方、文字列に UTF-8 として入ったときの byte 数は文字によって違います。

```rust
use std::mem::size_of;

assert_eq!(size_of::<char>(), 4);
assert_eq!(':'.len_utf8(), 1);
assert_eq!('あ'.len_utf8(), 3);
```

`&str` のスライス境界に必要なのは、`char` のサイズではなく UTF-8 上の byte 位置です。

## `char_indices()`

`&str` を安全に切り出したいときは、`char_indices()` が重要です。

```rust
for (i, ch) in s.char_indices() {
    println!("{i}: {ch}");
}
```

ここでの `i` は「その文字が始まる byte 位置」です。

`chars().enumerate()` の添字は文字番号なので、`&s[a..b]` の添字にはそのまま使えません。

## 文字列切り出しの例

```rust
fn first_word(s: &str) -> Option<&str> {
    let mut start = None;

    for (i, ch) in s.char_indices() {
        if ch.is_whitespace() {
            if let Some(start) = start {
                return Some(&s[start..i]);
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }

    start.map(|start| &s[start..])
}
```

役割分担:

- 文字として判定する: `ch.is_whitespace()`
- スライス位置を取る: `char_indices()` の byte index

C なら byte pointer を直接進めますが、Rust の `&str` では byte 位置と文字を分けて考える必要があります。

## `Vec<T>` のメモリ像

`Vec<T>` は可変長配列です。

```rust
let mut v = Vec::new();
v.push(10);
v.push(20);
v.push(30);
```

感覚としては:

- スタック: `Vec` の管理情報
- ヒープ: 実データ
- 管理情報: ポインタ、長さ、容量に近いもの

`push` で再確保が起きることがあります。そのため、要素への参照を持ったまま `push` できない場合があります。内部バッファの位置が変わる可能性があるためです。

`v[index]` は範囲外なら panic します。`v.get(index)` は `Option<&T>` を返します。

```rust
match v.get(10) {
    Some(x) => println!("{x}"),
    None => println!("no such element"),
}
```

## `Box<T>`

`Box<T>` は値を heap に置き、stack には pointer を置く所有ポインタです。

C の `malloc` した `T*` を所有する RAII wrapper に近いです。

```rust
let x = Box::new(10);
println!("{x}");
```

`String`, `Vec<T>`, `Box<T>` に共通して重要なのは、どの値が heap を所有しているかです。参照は所有者より長生きできません。

## lifetime の基礎

lifetime は参照の有効期間です。最初から注釈を書ける必要はありませんが、「参照は参照先より長生きできない」と理解するのが重要です。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` は「戻り値は、入力参照の両方に共通して有効な期間の中でしか使えない」という関係を表します。

lifetime 注釈は値の寿命を伸ばしません。参照同士の関係をコンパイラへ説明するだけです。

## ダングリング参照は止まる

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

これはコンパイルできません。`s` は内側のスコープで破棄されるので、その参照を外で使うと dangling になるからです。

C なら通ってしまい、未定義動作になります。Rust はここをコンパイル時に止めます。

## `'static`

`'static` はプログラム全体で有効な参照を表します。文字列リテラルは代表例です。

```rust
let s: &'static str = "hello";
```

ただし、困ったときに何でも `'static` にすればよいわけではありません。むやみに `'static` を要求している場合、所有権設計をごまかしていることがあります。
