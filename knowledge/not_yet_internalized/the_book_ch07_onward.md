# The Book 7章以降のまだ曖昧な知識

対象: https://doc.rust-jp.rs/book-ja/

このノートは、The Rust Programming Language 日本語版の 7章以降について、利用者が「まだ曖昧な知識」と明示したため `not_yet_internalized/` に置いている整理です。

## 7章: package, crate, module

既存ノートとの差分:

- module はコードの名前空間とプライバシー境界を作る。
- `mod front_of_house { ... }` のように module tree を定義する。
- 子 module は親 module の要素を直接見られるが、親から子の private 要素は見えない。
- `pub mod`, `pub fn`, `pub struct`, `pub enum` のように公開範囲を個別に指定する。
- `pub struct` でもフィールドはデフォルト private。公開したいフィールドには `pub` が必要。
- `pub enum` の variant はまとめて公開される。
- `super::` は親 module から始める相対パス。
- `pub use path::Item;` は再エクスポート。内部構造と公開 API の見せ方を分けられる。
- 外部 package は `Cargo.toml` に依存を追加し、コードでは crate 名からパスを書く。
- `use std::{cmp::Ordering, io};` のように nested path でまとめられる。
- `use std::io::{self, Write};` は module 自体とその中の item を同時に持ち込む形。
- glob operator `*` はテストや prelude 的用途では便利だが、通常は何が入るか見えにくい。
- ファイル分割は `mod garden;` と `src/garden.rs`、または `src/garden/mod.rs` の対応で行う。

## 8章: collections

### `Vec<T>`

未整理ポイント:

- `Vec::new()` または `vec![...]` で作る。
- `push` するには `let mut v` が必要。
- `v[index]` は範囲外なら panic、`v.get(index)` は `Option<&T>` を返す。
- `Vec<T>` の要素への参照を持ったまま `push` できない場合がある。再確保で内部バッファの位置が変わる可能性があるため。
- `Vec<T>` は同じ型の値だけを持つ。異なる型を入れたい場合は enum で包む、または trait object を使う。
- `Vec<T>` が drop されると要素も drop される。

### `String`

未整理ポイント:

- `String` は UTF-8 の可変バッファ。
- `String::new()`, `"text".to_string()`, `String::from("text")` で作れる。
- `push_str(&str)` は文字列 slice を追加し、`push(char)` は1文字を追加する。
- `s1 + &s2` は `s1` をムーブして、`&s2` を追加する。`s1` は以後使えない。
- 複数の文字列結合は `format!` が読みやすいことが多い。
- `String` は添字で1文字を取れない。UTF-8 では byte index と文字境界が一致しないため。
- bytes と chars と grapheme cluster は別概念。標準の `chars()` は Unicode scalar value 単位。

### `HashMap<K, V>`

未整理ポイント:

- `HashMap::new()` で作り、`insert` で追加する。
- `get(&key)` は `Option<&V>`。
- 所有値を `insert` すると map にムーブされる。`Copy` 型ならコピーされる。
- `entry(key).or_insert(value)` は「なければ挿入し、あれば既存値への可変参照を返す」。
- 既存値を更新する word count のような処理では `let count = map.entry(word).or_insert(0); *count += 1;` が典型。
- 標準の `HashMap` は DoS 耐性を意識した hasher を使う。最高速度だけが目的なら別 hasher を選ぶ余地がある。

## 9章: error handling

既存ノートとの差分:

- `panic!` は回復不能エラーを表す。デフォルトでは stack unwind して drop を走らせる。
- `Cargo.toml` の profile で `panic = 'abort'` にすると unwind せず即終了し、バイナリを小さくできる場合がある。
- `RUST_BACKTRACE=1` で backtrace を出せる。
- `Result<T, E>` の `match` で `ErrorKind::NotFound` などエラー種別に応じた処理ができる。
- `unwrap_or_else` は error handling の分岐を closure に閉じ込められる。
- `?` は `Err` を呼び出し元へ返す。内部的には `From` によるエラー型変換が絡む。
- `?` は `Result` を返す関数、または対応する戻り値型の関数で使う。
- `main() -> Result<(), Box<dyn std::error::Error>>` とすると、いろいろなエラーを `?` で上げやすい。
- panic すべき場面は、契約違反、回復不能、サンプルやテスト、プロトタイプなど。
- ユーザー入力、I/O、ネットワークの失敗は普通に起こるので `Result` で返すのが自然。

## 10章: generics, traits, lifetimes

既存ノートとの差分:

- ジェネリクスは関数、struct、enum、method に使える。
- `impl<T> Point<T>` は全ての `T` に対する実装。`impl Point<f32>` のように特定型だけに method を生やせる。
- ジェネリクスは monomorphization により具体型ごとのコードに展開されるため、通常は実行時コストを増やさない。
- trait には default implementation を書ける。実装側は必要な method だけ上書きできる。
- default method から、同じ trait の必須 method を呼べる。
- trait を引数に使う書き方は複数ある。
  - `fn f(x: &impl Summary)`
  - `fn f<T: Summary>(x: &T)`
  - `fn f<T>(x: &T) where T: Summary`
- `impl Trait` を戻り値に使うと「ある1つの具体型を返すが、呼び出し側には trait として見せる」という意味になる。分岐ごとに別の具体型を返す用途ではない。
- blanket implementation は、条件を満たすすべての型に trait を実装する形。例: `impl<T: Display> ToString for T` のような発想。
- lifetime 注釈は参照同士の関係をコンパイラに伝えるもので、値の寿命を伸ばさない。
- dangling reference を避けるため、戻り値参照がどの引数由来か曖昧なときに lifetime が必要になる。
- lifetime elision rules により、単純な関数では省略できる。
- method の `&self` / `&mut self` がある場合、戻り値参照の lifetime は多くの場合 `self` 由来と推論される。
- `'static` はプログラム全体で有効な参照を表す。ただし、むやみに `'static` を要求すると所有権設計をごまかしていることがある。

## 11章: tests

未整理ポイント:

- `#[cfg(test)] mod tests { ... }` の中にテストを書くのが典型。
- `#[test]` を付けた関数がテストとして実行される。
- `assert!`, `assert_eq!`, `assert_ne!` が基本。
- assert macro には追加メッセージを渡せる。
- `#[should_panic]` は panic することを期待するテスト。`expected = "..."`
  で panic message の一部も確認できる。
- テスト関数は `Result<(), E>` を返して `?` を使ってもよい。
- `cargo test test_name` で名前フィルタをかけられる。
- `#[ignore]` は重いテストなどを通常実行から外す。`cargo test -- --ignored` で実行する。
- unit test は同じファイル内の private item もテストできる。
- integration test は `tests/` ディレクトリに置き、外部 crate として公開 API をテストする。
- binary crate だけだと integration test しづらいので、ロジックは library crate に分けるとよい。

## 12章: CLI project

未整理ポイント:

- `std::env::args()` は command line arguments の iterator を返す。
- 小さいプログラムでも、引数解析、ファイル読み込み、検索ロジック、出力を分けるとテストしやすい。
- `main.rs` は orchestration、`lib.rs` は testable logic に寄せる。
- 設定を `Config` struct にまとめると、引数の意味が明確になる。
- `Box<dyn Error>` は「いろいろなエラー型をまとめて返す」ための簡便な型として使える。
- TDD の流れは、失敗するテストを書く、通る最小実装を書く、リファクタする。
- 環境変数は `std::env::var("NAME")` で読む。失敗は `Result`。
- エラーや診断は `eprintln!` で stderr に出す。通常出力と混ぜない。
- `process::exit(1)` は異常終了 status を返す。

## 13章: closures and iterators

### closures

未整理ポイント:

- closure は匿名関数で、周囲の変数を capture できる。
- closure の引数型や戻り値型は多くの場合推論されるが、一度推論された型で固定される。
- capture は状況に応じて共有借用、可変借用、所有権の取得になる。
- `move` closure は使う値の所有権を closure に移す。thread に渡す closure でよく使う。
- closure は capture の仕方に応じて `Fn`, `FnMut`, `FnOnce` の trait を実装する。
- `FnOnce` は capture した値を消費できるので一度だけ呼べる。
- `FnMut` は可変に capture した状態を変更できる。
- `Fn` は共有参照だけで呼べる。

### iterators

未整理ポイント:

- iterator は lazy。消費する操作を呼ぶまで何もしない。
- `next()` は iterator の状態を進めるので、通常 `mut` が必要。
- `sum`, `collect` などは consuming adaptor。
- `map`, `filter` などは iterator adaptor。別の iterator を返すだけなので、最後に消費しないと実行されない。
- iterator を使ったコードは、最適化後に手書き loop と同等の性能になることを狙って設計されている。

## 14章: Cargo and crates.io

未整理ポイント:

- release profile は `[profile.dev]`, `[profile.release]` で設定できる。例: `opt-level`。
- documentation comment は `///`。Markdown として rustdoc に取り込まれる。
- doc comment 内の Rust code block は doctest として実行できる。
- crate root の `//!` は、その item 自体ではなく、それを含む item へのドキュメント。
- `pub use` は API 表面を整えるためにも使う。
- crates.io に公開するには metadata、license、description などを `Cargo.toml` に書く。
- `cargo publish` で公開し、公開済み version は上書きできない。
- `cargo yank` は既存 version を新規依存解決から外すが、既存 lockfile のビルドは壊さない。
- workspace は複数 package の `Cargo.lock` と `target/` を共有できる。
- `cargo install` は binary crate をインストールする仕組み。library crate を依存として使う操作ではない。
- `cargo-something` という実行ファイルが PATH にあると、`cargo something` として呼べる。

## 15章: smart pointers

未整理ポイント:

- smart pointer はポインタのように振る舞うが追加の metadata や振る舞いを持つ型。
- `Box<T>` は値を heap に置き、stack には pointer を置く。
- recursive type はそのままだとサイズが無限になるので、`Box` などで間接化する。
- `Deref` trait を実装すると `*x` の振る舞いを定義できる。
- deref coercion により、`&String` を `&str` が必要な関数へ渡せる。
- `DerefMut` は可変参照向け。
- `Drop` trait は値がスコープを抜けるときの片付けを定義する。明示的に早く drop したい場合は `std::mem::drop(value)`。
- `Rc<T>` は単一 thread 内の参照カウント共有所有権。
- `Rc::clone(&x)` は深いコピーではなく参照カウント増加。
- `RefCell<T>` は borrow rule を実行時に検査する内部可変性。違反すると panic。
- `Rc<RefCell<T>>` は「複数所有 + 中身の可変化」を可能にするが、設計が複雑になりやすい。
- `Rc<T>` の循環参照はメモリリークを起こしうる。
- `Weak<T>` は所有権を持たない弱参照。親子関係などで cycle を避けるために使う。

C との対応:

- `Box<T>` は `malloc` した `T*` を所有する RAII wrapper に近い。
- `Rc<T>` は参照カウント付き pointer。
- `RefCell<T>` は borrow checker を実行時チェックに移す箱。ただし thread-safe ではない。

## 16章: concurrency

未整理ポイント:

- `thread::spawn(|| { ... })` で新しい thread を開始する。
- `JoinHandle::join()` で thread の終了を待つ。
- thread closure が外側の値を使う場合、`move` で所有権を移すことが多い。
- channel は `mpsc::channel()` で作る。multiple producer, single consumer。
- `tx.send(value)` は値を channel にムーブする。
- receiver は iterator として扱える。
- `tx.clone()` で producer を増やせる。
- 共有状態には `Mutex<T>` を使う。`lock()` は `MutexGuard` を返し、guard が drop されると unlock される。
- 複数 thread で共有するには `Arc<T>` を使う。`Rc<T>` は thread-safe ではない。
- 典型形は `Arc<Mutex<T>>`。
- `Send` は所有権を thread 間で移せる型、`Sync` は複数 thread から参照してよい型を表す marker trait。
- 多くの型では `Send` / `Sync` は自動導出されるが、raw pointer や interior mutability を持つ型では注意が必要。

## 17章: object oriented features

既存ノートとの差分:

- Rust では `pub` によって encapsulation を作る。フィールドを private にして method 経由で不変条件を守る。
- 継承より、trait による共有振る舞いとジェネリクス/trait object による多態性を使う。
- `Vec<Box<dyn Draw>>` のようにすると、異なる具体型を同じ collection に入れられる。
- trait object は dynamic dispatch なので、呼び出す method は実行時に vtable 経由で決まる。
- trait object にできる trait には制約がある。典型的には、戻り値に `Self` を使う method や generic method は object safety に引っかかる。
- state pattern は trait object で実装できるが、Rust では状態を型で表す type state pattern の方がコンパイル時に不正状態を消せる場合がある。

## 18章: patterns and matching

既存ノートとの差分:

- pattern が出る場所は `match` arms, `if let`, `while let`, `for`, `let`, 関数引数など。
- pattern には refutable と irrefutable がある。
- `let` の左辺には常に一致する irrefutable pattern が必要。
- `if let` や `while let` は失敗しうる refutable pattern を受け付ける。
- literal pattern, named variable pattern, multiple pattern `|`, range pattern `..=` がある。
- pattern 内の変数名は外側の同名変数を shadowing する。
- struct, enum, tuple を pattern で destructure できる。
- `_` は値を束縛しない。`_x` は束縛するが未使用警告を抑える。
- `..` は残りのフィールドや要素を無視する。
- match guard `Some(x) if x % 2 == 0` のように追加条件を付けられる。
- `@` binding は「pattern に一致させつつ、その値全体にも名前を付ける」。

## 19章: advanced features

### unsafe Rust

未整理ポイント:

- `unsafe` は borrow checker を止めるスイッチではなく、5つの追加能力を許す境界。
- unsafe でできること:
  - raw pointer の dereference
  - unsafe function / method の呼び出し
  - mutable static variable へのアクセスや変更
  - unsafe trait の実装
  - union field へのアクセス
- raw pointer は `*const T` と `*mut T`。null や dangling や aliasing を作れてしまう。
- unsafe function は呼び出し側が追加の契約を守る必要がある関数。
- safe abstraction の内側に unsafe を閉じ込めるのが基本。
- FFI では `extern "C"` で C ABI の関数を宣言・公開する。
- global mutable state は競合や未定義動作の原因になりやすい。

### advanced traits

未整理ポイント:

- associated type は trait 内で関連する型を1つ決める仕組み。`Iterator::Item` が代表例。
- generic type parameter と associated type は似ているが、associated type は実装ごとに1つに固定される。
- default generic type parameter は `Add<Rhs = Self>` のようにデフォルト型を持たせる仕組み。
- operator overloading は `std::ops` の trait を実装して行う。
- 同名 method が複数 trait / inherent impl にあるときは、fully qualified syntax で呼び分ける。
- supertrait は「この trait を実装するには別 trait も必要」という制約。
- newtype pattern は既存型を tuple struct で包み、外部 trait を実装したり型の意味を分けたりする手法。

### advanced types

未整理ポイント:

- type alias は既存型に別名を付ける。新しい別型を作るわけではない。
- never type `!` は戻らない計算の型。`panic!`, `loop` などと関係する。
- dynamically sized type はサイズがコンパイル時に分からない型。`str`, `[T]`, `dyn Trait` など。
- generic type parameter はデフォルトで `Sized` を要求する。緩めるには `T: ?Sized`。

### advanced functions and closures

未整理ポイント:

- function pointer 型は `fn(i32) -> i32`。
- function pointer は `Fn`, `FnMut`, `FnOnce` の3 trait すべてを実装する。
- closure を返すときは、具体型を書けないので `impl Fn(...) -> ...` や `Box<dyn Fn(...) -> ...>` を使う。

### macros

未整理ポイント:

- macro は Rust code を生成する code writing code。
- `macro_rules!` は declarative macro。
- `vec![1, 2, 3]` のように、可変個引数や特殊構文を扱える。
- procedural macro は入力 token stream から出力 token stream を作る。
- procedural macro には custom derive, attribute-like macro, function-like macro がある。

## 20章: multithreaded web server

未整理ポイント:

- `TcpListener::bind("127.0.0.1:7878")` で port を listen する。
- `listener.incoming()` は接続 stream の iterator。
- `TcpStream` から request を読み、HTTP response を書く。
- HTTP はまず text protocol として扱える。学習用には request line を読んで path を分岐する。
- single-thread server は1つの遅い request で後続 request が詰まる。
- thread pool は worker thread を先に作り、job を channel で配る。
- job は `Box<dyn FnOnce() + Send + 'static>` のように表せる。
- graceful shutdown では sender を drop して channel を閉じ、worker に終了を知らせ、`join` で待つ。
- `Drop` 実装で worker thread の終了処理をまとめられる。

## 付録

未整理ポイント:

- 付録Aは keyword 一覧。現在使う keyword、将来予約、raw identifier などを区別する。
- 付録Bは operator と記号一覧。所有権・借用・path・generic・lifetime・range などの記号を横断的に確認できる。
- 付録Cは derive 可能 trait。`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Default` など。
- 付録Dは rustfmt, rustfix, clippy, rust-analyzer などの開発支援ツール。
- 付録Eは edition。edition は互換性を保ちながら構文や慣用を進める仕組み。
- 付録Gは stable / beta / nightly と RFC による Rust の作られ方。

## 次に個別ノートへ分割する候補

再利用価値が高いので、今後質問や演習で出てきたら個別ノートに切り出すとよい候補:

- `cargo_check_build_run.md`: `cargo check` / `build` / `run` / `test` / `release` の使い分け
- `vec_string_hashmap.md`: collection の所有権、借用、更新
- `testing_in_rust.md`: unit test / integration test / `should_panic` / `Result` test
- `closures_fn_traits.md`: closure capture と `Fn` / `FnMut` / `FnOnce`
- `smart_pointers.md`: `Box` / `Rc` / `RefCell` / `Weak`
- `send_sync_arc_mutex.md`: thread, channel, `Arc<Mutex<T>>`, `Send` / `Sync`
- `unsafe_boundaries.md`: unsafe の5能力と safe abstraction
- `advanced_patterns.md`: refutability, guard, `@`, `..`
- `cargo_publish_docs.md`: rustdoc, doctest, crates.io, workspace 公開設計
