# Rust 基礎と Cargo

このノートは、C 経験者が Rust を読むときの最初の地図です。`rustc` / `cargo`、変数、基本型、式、制御構文、関数、最初の学習順序をまとめます。

## Rust を C と比べると

Rust は「低レベルの感覚を保ちつつ、C で実行時まで残りやすいメモリ破壊やデータ競合をコンパイル時に潰す」言語です。

C で人間が追うこと:

- 解放後アクセス
- 二重 `free`
- NULL 参照
- バッファオーバーラン
- 共有可変データの競合

Rust ではかなりの部分を型システムと borrow checker が止めます。その代わり、最初はコンパイラがかなり細かく指摘します。この「うるささ」が Rust の安全性の入口です。

## `rustc` と `cargo`

C で言うと:

- `gcc` / `clang`: コンパイラ
- `make`: ビルド管理

Rust で言うと:

- `rustc`: 単体コンパイル
- `cargo`: ビルド、依存管理、テスト、実行、ドキュメント生成

普段は `cargo` 中心で考えるのが実用的です。

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

よく使うコマンド:

- `cargo run`: ビルドして実行
- `cargo check`: 実行ファイルを作らず、型と所有権を高速確認
- `cargo test`: テスト実行
- `cargo build`: ビルド
- `cargo build --release`: 最適化ビルド

`cargo check` は、C で言えば「リンクや成果物生成より先に、型と構造の整合性を高速に検査する」道具です。

## `Cargo.toml` と `Cargo.lock`

`Cargo.toml` は package 情報と直接依存を書きます。

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

`Cargo.lock` は、実際に解決された依存バージョンを固定します。アプリケーションでは lockfile をコミットするのが普通です。

## crate, package, workspace

用語は混ざりやすいので、最初は次で十分です。

- crate: Rust のコンパイル単位
- package: Cargo が扱う 1 個のプロジェクト単位
- workspace: 複数 package をまとめて管理する単位

`src/main.rs` から始まるものは binary crate で、実行ファイルになります。

`src/lib.rs` から始まるものは library crate で、他の crate から使われる部品になります。

binary crate は「最終成果物として実行ファイルを作る crate」です。C の `.o` のようなオブジェクトファイルを指す言葉ではありません。

1 個の package は、0 個か 1 個の library crate と、0 個以上の binary crate を持てます。

library crate が 1 つだけなのは、package 名で公開される主要ライブラリを 1 つに決める Cargo の設計です。複数のライブラリに分けたい場合は、通常は複数 package を workspace に置きます。

Rust は基本的に crate 単位でコンパイルするため、同じ crate 内の 1 ファイルを変更すると、その crate は再コンパイル対象になります。C のように `.c` ごとの `.o` だけを差し替えるモデルとは違います。ただし依存 crate は変更がなければ再利用されます。

workspace は複数 package をまとめます。

```toml
[workspace]
members = [
    "00_strlen",
    "01_max_option",
]
```

このリポジトリでは、リポジトリ直下には Cargo workspace はなく、`ex/ex01`, `ex/ex02_traits`, `ex/ex03_review` がそれぞれ独立した workspace です。

## 変数と `mut`

Rust の変数はデフォルトで不変です。

```rust
fn main() {
    let x = 10;
    // x = 20; // エラー

    let mut y = 10;
    y = 20;
    println!("{y}");
}
```

`mut` は「その束縛を通じて値を変更できる」という意味です。C は `const` を付けない限り可変ですが、Rust は逆です。

可変性を明示すると、どこで状態が変わるかを追いやすくなります。

## シャドーイング

```rust
fn main() {
    let x = 10;
    let x = x + 1;
    let x = x * 2;
    println!("{x}");
}
```

これは再代入ではなく、新しい変数の再束縛です。型を変えても構いません。

```rust
let input = "42";
let input: u32 = input.parse().unwrap();
```

「文字列として読んだ値を、同じ名前で数値へ変換する」ような場面でよく使います。

## `const`

`const` は常に不変で、型注釈が必須です。

```rust
const MAX_POINTS: u32 = 100_000;
```

`let` の不変変数とは違い、コンパイル時定数として扱います。

## 基本型

整数型:

- 符号付き: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- 符号なし: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

`usize` / `isize` はマシンのポインタ幅に合う型です。配列、slice、`Vec` の長さや添字でよく出ます。

整数リテラル:

```rust
let a = 98_222;
let b = 0xff;
let c = 0o77;
let d = 0b1111_0000;
let e = b'A';
```

デバッグビルドの整数オーバーフローは panic します。リリースビルドではラップします。明示的に扱うなら `wrapping_*`, `checked_*`, `overflowing_*`, `saturating_*` を使います。

その他:

- `bool`: `true` / `false`
- `char`: Unicode scalar value。C の 1 byte `char` とは違う
- tuple: 固定長で、要素ごとに型を変えられる
- `()`: unit 型。C の `void` に少し近いが、値として存在する

```rust
let t: (i32, f64, char) = (10, 3.14, 'A');
let unit: () = ();
```

## 文と式

Rust は式ベースの言語です。関数の最後の式は、セミコロンを付けなければ戻り値になります。

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

セミコロンを付けると文になり、値を返しません。

```rust
fn bad_add(a: i32, b: i32) -> i32 {
    a + b; // 戻り値は () になるのでエラー
}
```

## `if` は式

```rust
fn abs_diff(a: i32, b: i32) -> i32 {
    if a > b { a - b } else { b - a }
}
```

`if` は値を返せます。

```rust
let x = if condition { 1 } else { 2 };
```

ただし、両腕の型は同じである必要があります。

## ループ

Rust には `loop`, `while`, `for` があります。

`loop` は無限ループです。`break value` で値を返せます。

```rust
let mut n = 0;
let answer = loop {
    n += 1;
    if n == 10 {
        break n * 2;
    }
};
```

ネストしたループではラベルを付けられます。

```rust
'outer: loop {
    loop {
        break 'outer;
    }
}
```

`while` は条件が成り立つ間だけ繰り返します。

```rust
let mut n = 100;

while n > 1 {
    n /= 2;
}
```

`for` は範囲やコレクションの要素を順に取り出します。C の `for (i = 0; i < n; i++)` というより、「列を順に走査する構文」です。

```rust
for i in 0..5 {
    println!("{i}");
}
```

`0..5` は `0, 1, 2, 3, 4` を作ります。`0..=5` は `5` も含みます。

## 関数

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

引数型は基本的に明示します。戻り値があるときは `-> Type` を書きます。

所有権が不要なら、引数は `&T` や `&str` にすると呼び出し側の値を消費しません。

## 数当てゲームで出てくる初期要素

The Book 2章の数当てゲームは、最初にいろいろな Rust 要素を浅く見せます。

- `use rand::Rng;`: 外部 crate の trait をスコープに入れる
- `std::cmp::Ordering`: `Less`, `Greater`, `Equal` を持つ enum
- `match guess.cmp(&secret_number)`: 比較結果を enum として受けて分岐
- `trim()`: 末尾の改行などを落とす
- `parse()`: 文字列を数値などへ変換する。変換先の型情報が必要
- `continue`: 次のループ周回へ進む
- `break`: ループを抜ける

この段階では全部を理解し切るより、Rust が「型」と「失敗」を追跡している感覚を掴むのが目的です。

## 最初に覚える標準ライブラリ

優先度順:

- `String`
- `Vec<T>`
- `Option<T>`
- `Result<T, E>`
- `std::fs`
- `std::collections::HashMap`

最初の段階では、外部 crate より標準ライブラリ中心で十分です。

## 学習順序

無理が少ない順序:

1. `let`, `mut`, 基本型
2. `String`, `&str`, `Vec`
3. 所有権と借用
4. `struct` と `impl`
5. `enum`, `match`, `Option`, `Result`
6. スライス
7. ライフタイム
8. 必要になってから `trait`, generics, `unsafe`

最初から高度な lifetime 注釈、trait object、非同期、マクロ自作、高度な並行処理、`unsafe` の細部を深追いしなくて構いません。

## C との対応

- `struct`: Rust の `struct`
- `enum + union`: Rust のデータ付き `enum`
- `char *`: 文脈次第で `String`, `&str`, `Vec<u8>`
- `malloc/free`: 所有権 + 自動破棄
- `NULL`: `Option<T>`
- 戻り値 + `errno`: `Result<T, E>`
- `const T *`: `&T` に少し近い
- `T *`: `&mut T` や raw pointer `*mut T` に分かれる

完全な 1 対 1 対応ではありません。Rust は「どの操作が安全か」を型で細かく分けています。
