# `crate`, `package`, `workspace`

このノートは、Rust を学び始めたときに混ざりやすい
`crate`, `package`, `workspace` の違いを切り分けるためのものです。

## まず一言で

- `crate`: Rust のコンパイル単位
- `package`: Cargo が扱う 1 個のプロジェクト単位
- `workspace`: 複数 package をまとめて管理する単位

最初は次の形で覚えると十分です。

- `src/main.rs` なら binary crate
- `src/lib.rs` なら library crate
- `Cargo.toml` の `[package]` は package
- `Cargo.toml` の `[workspace]` は workspace

## C と比べると

ざっくりした感覚は次です。

- crate: C の「最終的に 1 個のライブラリや実行ファイルになるまとまり」に近い
- package: そのまとまりを Cargo で管理するディレクトリ
- workspace: 複数の小さなプロジェクトをまとめる親ディレクトリ

ただし crate は C の 1 個の `.c` ファイルとは違います。
C の translation unit は 1 個のソースファイル単位ですが、
Rust の crate は複数モジュールを含んだまま 1 つの単位としてコンパイルされます。

## `crate` とは

`crate` は Rust のコンパイル単位です。

たとえば:

- `src/main.rs` から始まるものは binary crate
- `src/lib.rs` から始まるものは library crate

```rust
fn main() {
    println!("hello");
}
```

これは binary crate の入口です。

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

これは library crate の中に置く関数の例です。

## binary crate と library crate

### binary crate

- 実行ファイルになる
- `fn main()` がある
- `cargo run` で動かすことが多い

### library crate

- 他の crate から使われる部品になる
- `fn main()` は不要
- `cargo test` や、別 crate からの利用で確認することが多い

## `package` とは

`package` は Cargo が扱う 1 個のプロジェクトです。
ふつうは 1 個の `Cargo.toml` に対応します。

例えば:

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2024"
```

この `[package]` がある `Cargo.toml` は、
Cargo から見て「1 個の package」です。

package は中に crate を持ちます。
最初は次の理解で十分です。

- `src/main.rs` があれば 1 個の binary crate を持つ
- `src/lib.rs` があれば 1 個の library crate を持つ

少し正確に言うと、
1 個の package は「0 個か 1 個の library crate」と
「0 個以上の binary crate」を持てます。

## package と crate の違い

ここが一番混ざりやすいところです。

- package: Cargo の管理単位
- crate: Rust のコンパイル単位

つまり:

- `Cargo.toml` で見ると package
- `main.rs` / `lib.rs` で見ると crate

です。

最初は「ふだん触るディレクトリは package、
その中で実際にコンパイルされる本体が crate」
と思っておくと整理しやすいです。

## `workspace` とは

`workspace` は複数 package をまとめる仕組みです。

例えば親ディレクトリにこういう `Cargo.toml` を置きます。

```toml
[workspace]
members = [
    "00_strlen",
    "01_max_option",
    "02_stack",
]
```

このとき親ディレクトリは workspace root です。
その下の各 member はそれぞれ独立した package です。

workspace を使う主な理由は次です。

- 関連する crate をひとまとめに管理したい
- `cargo test --workspace` のようにまとめて実行したい
- `Cargo.lock` や `target/` を共有したい

C でたとえるなら、
小さなライブラリやツールを複数持つリポジトリで
上位の Makefile やビルド管理を共有する感じに近いです。

## このリポジトリでの実例

### 単独 package の例

`ex/ex00/hello_rust` は単独 package です。

- `Cargo.toml` に `[package]` がある
- `src/main.rs` がある
- つまり「1 個の binary crate を持つ package」

### workspace の例

`ex/ex01` は workspace root です。

- `ex/ex01/Cargo.toml` に `[workspace]` がある
- member に `00_strlen`, `01_max_option` などが並ぶ

その中の `ex/ex01/00_strlen` は member package です。

- `Cargo.toml` に `[package]` がある
- `[lib]` で `src/lib.rs` を使っている
- つまり「library crate を持つ package」

関係を図で書くとこうです。

```text
ex/ex01/                 <- workspace
  Cargo.toml
  00_strlen/             <- package
    Cargo.toml
    src/lib.rs           <- library crate
  01_max_option/         <- package
    Cargo.toml
    src/lib.rs           <- library crate
```

## どう使い分けるか

最初は次の基準で十分です。

### 1. 小さい練習やコマンド 1 本なら単独 package

- 例: `hello_rust`
- `cargo new hello_rust`

### 2. 再利用したい処理なら library crate

- 他のコードから呼びたい
- テストを書きたい
- ロジックを `main` から分けたい

### 3. 関連する複数課題や複数 crate をまとめるなら workspace

- 演習を章ごとにまとめたい
- 複数 package を一括で `check` / `test` したい

## よく使うコマンド

### 単独 package で

```bash
cargo run
cargo check
cargo test
```

### workspace root で

```bash
cargo check --workspace
cargo test --workspace
```

### workspace 内の特定 package だけ

```bash
cargo test -p ex00_strlen
```

`-p` は package 名を指定します。
ディレクトリ名ではなく `Cargo.toml` の `name` を見る点に注意です。

## `crate::` は何を指すのか

コード中で出てくる `crate::foo::bar` の `crate` は、
「今いる crate のルートから見たパス」です。

これは workspace 全体を指しません。
今コンパイルしているその crate だけを指します。

たとえば:

```rust
use crate::util::parse;
```

は「同じ crate の中にある `util::parse` を使う」です。

別 package や別 crate を指したいなら、
依存関係を宣言したうえでその crate 名から始めます。

## 今の段階での理解のゴール

- crate はコンパイル単位
- package は Cargo の管理単位
- workspace は複数 package のまとめ役
- `src/main.rs` は binary crate、`src/lib.rs` は library crate
- `crate::` は「今の crate の先頭から」の意味
