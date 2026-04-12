# Rust Practice 50 Questions

`knowledge/` と `knowledge/rust_from_c_guide.md` の範囲を、実際に手を動かして定着させるための課題集です。
既存の [`rust_review_questions.md`](./rust_review_questions.md) より少し広く、trait や module まわりまで含めています。

## 進め方

1. まずは上から順に解く
2. 1問ごとに小さな crate か 1 ファイルを作る
3. できれば各問で `#[test]` を 2 個以上書く
4. 最初は iterator メソッド連鎖より、`for` / `match` / `while let` を優先してよい

## 1. 所有権・借用・`mut`

### 01. `borrowed_len`

- 主に定着: `String` を受け取らなくても、読むだけなら `&str` で十分だという感覚
- 課題: `fn borrowed_len(s: &str) -> usize` を書く
- 条件: 長さを返すだけにし、新しい `String` は作らない

### 02. `append_exclamation`

- 主に定着: `&mut String` で「所有権は渡さずに書き換える」感覚
- 課題: `fn append_exclamation(s: &mut String)` を書く
- 条件: 末尾が `!` でなければ `!` を 1 個だけ足す

### 03. `take_and_report`

- 主に定着: `String` を値で受け取ると所有権が移ること
- 課題: `fn take_and_report(s: String) -> (String, usize)` を書く
- 条件: 受け取った文字列とその長さを返す

### 04. `longer_len`

- 主に定着: 複数の借用を同時に読むだけなら問題ないこと
- 課題: `fn longer_len(a: &str, b: &str) -> usize` を書く
- 条件: 長い方の長さを返す。長さが同じならどちらでもよい

### 05. `count_positive`

- 主に定着: `&[i32]` を読むだけの関数に慣れること
- 課題: `fn count_positive(xs: &[i32]) -> usize` を書く
- 条件: 0 より大きい要素の個数を返す

### 06. `negate_all`

- 主に定着: `&mut [i32]` をその場で更新する感覚
- 課題: `fn negate_all(xs: &mut [i32])` を書く
- 条件: すべての要素を符号反転する

### 07. `take_last`

- 主に定着: `Vec<T>` の所有権を保ったまま、末尾だけを取り出す操作
- 課題: `fn take_last(v: &mut Vec<i32>) -> Option<i32>` を書く
- 条件: 空なら `None`

### 08. `duplicate_last`

- 主に定着: いったん値を読み出してから push する、という借用の順序
- 課題: `fn duplicate_last(v: &mut Vec<i32>)` を書く
- 条件: 末尾要素があれば同じ値を 1 個追加する

## 2. 配列・スライス・`String`・`Vec`

### 09. `first_word`

- 主に定着: `&str` から新しい文字列を作らずに部分文字列を返すこと
- 課題: `fn first_word(s: &str) -> Option<&str>` を書く
- 条件: 空白で区切った最初の単語を返す。見つからなければ `None`

### 10. `split_once_colon`

- 主に定着: 1 個の `&str` から 2 個の `&str` を切り出す感覚
- 課題: `fn split_once_colon(s: &str) -> Option<(&str, &str)>` を書く
- 条件: 最初の `:` で 2 つに分ける。前半か後半が空なら `None`

### 11. `middle_two`

- 主に定着: `[T; N]` と `&[T]` の違い、および配列から slice を切り出す感覚
- 課題: `fn middle_two(xs: &[i32; 4]) -> &[i32]` を書く
- 条件: 真ん中 2 要素だけを slice として返す

### 12. `max_slice`

- 主に定着: `Option<T>` で「空配列では値がない」を表すこと
- 課題: `fn max_slice(xs: &[i32]) -> Option<i32>` を書く
- 条件: 空なら `None`

### 13. `reverse_in_place`

- 主に定着: 可変 slice をその場で更新すること
- 課題: `fn reverse_in_place(xs: &mut [i32])` を書く
- 条件: 新しい `Vec` は作らない

### 14. `collect_lengths`

- 主に定着: `&[String]` を読むだけで処理し、所有権を奪わないこと
- 課題: `fn collect_lengths(words: &[String]) -> Vec<usize>` を書く
- 条件: 各文字列の長さを順に入れた `Vec` を返す

### 15. `join_nonempty`

- 主に定着: `String` を組み立てるときの所有権と借用の使い分け
- 課題: `fn join_nonempty(words: &[&str]) -> String` を書く
- 条件: 空文字列を飛ばし、`,` でつないで返す

### 16. `drop_empty`

- 主に定着: `Vec<String>` をその場で絞り込む操作
- 課題: `fn drop_empty(v: &mut Vec<String>)` を書く
- 条件: 空文字列だけを取り除く

## 3. `struct`・`impl`・`derive`

### 17. `Counter`

- 主に定着: `struct` の状態を `impl` のメソッドで更新すること
- 課題: `struct Counter` を作り、`new`, `inc`, `get`, `reset` を実装する
- 条件: 内部は `i32` でよい

### 18. `Rectangle`

- 主に定着: `&self` で読むメソッドと、計算用メソッドの分離
- 課題: `struct Rectangle { width: u32, height: u32 }` を作る
- 条件: `area(&self) -> u32` と `is_square(&self) -> bool` を実装する

### 19. `BankAccount`

- 主に定着: `&mut self` と `Result` を組み合わせること
- 課題: `struct BankAccount { owner: String, balance: i64 }` を作る
- 条件: `deposit`, `withdraw`, `balance` を実装し、残高不足は `Err` にする

### 20. `Point`

- 主に定着: `&self` と `&mut self` の使い分け
- 課題: `struct Point { x: i32, y: i32 }` を作る
- 条件: `translate(&mut self, dx: i32, dy: i32)` と `manhattan_len(&self) -> i32` を実装する

### 21. `Book::new`

- 主に定着: `&str` から `String` を作って `Self` を返すこと
- 課題: `struct Book { title: String, pages: u32 }` を作る
- 条件: `fn new(title: &str, pages: u32) -> Self` を実装する

### 22. `Id`

- 主に定着: `#[derive(...)]` と `Copy` 型の扱い
- 課題: `#[derive(Debug, Clone, Copy, PartialEq, Eq)] struct Id(u32);` を作る
- 条件: `is_even(&self) -> bool` を実装し、比較テストも書く

### 23. `Stack`

- 主に定着: `Vec<T>` を内部に持つ型を自分で包むこと
- 課題: `struct Stack` を作り、`new`, `push`, `pop`, `peek`, `len` を実装する
- 条件: `peek` は `Option<&i32>` を返す

### 24. `Pair<T>`

- 主に定着: ジェネリクス付き `struct` と `self` を受け取るメソッド
- 課題: `struct Pair<T> { left: T, right: T }` を作る
- 条件: `swap(self) -> Pair<T>` を実装する

## 4. `enum`・`match`・`Option`・`Result`・パターン

### 25. `TrafficLight`

- 主に定着: 小さな `enum` を `match` で回すこと
- 課題: `enum TrafficLight { Red, Yellow, Green }` を作る
- 条件: `fn next(light: TrafficLight) -> TrafficLight` を実装する

### 26. `Packet`

- 主に定着: データ付き variant を `match` で分解すること
- 課題: `enum Packet { Ping, Message(String), Move { x: i32, y: i32 } }` を作る
- 条件: `fn describe(p: &Packet) -> String` を実装する

### 27. `safe_div`

- 主に定着: `Option<T>` で「失敗するかもしれない計算」を表すこと
- 課題: `fn safe_div(a: i32, b: i32) -> Option<i32>` を書く
- 条件: `b == 0` なら `None`

### 28. `parse_port`

- 主に定着: 文字列から数値への変換と `Result<T, E>`
- 課題: `fn parse_port(s: &str) -> Result<u16, String>` を書く
- 条件: 数値でない場合と `0` の場合は `Err`

### 29. `sum_present`

- 主に定着: `Option<i32>` の配列を `match` や `if let` で処理すること
- 課題: `fn sum_present(xs: &[Option<i32>]) -> i32` を書く
- 条件: `Some(v)` だけを合計する

### 30. `collect_ok`

- 主に定着: `Result<T, E>` から成功値だけを集めること
- 課題: `fn collect_ok(xs: &[Result<i32, &str>]) -> Vec<i32>` を書く
- 条件: `Err` は無視する

### 31. `pop_all`

- 主に定着: `while let` の基本形
- 課題: `fn pop_all(stack: &mut Vec<i32>) -> Vec<i32>` を書く
- 条件: `pop()` を繰り返し、取り出した順に返す

### 32. `first_error`

- 主に定着: 借用を保ったまま `Result` の中身を見ること
- 課題: `fn first_error<'a>(xs: &'a [Result<i32, &'a str>]) -> Option<&'a str>` を書く
- 条件: 最初の `Err` の文字列を返す

### 33. `Token`

- 主に定着: 複数の値をまとめて `match` すること
- 課題: `enum Token { Int(i32), Plus, Minus }` を作る
- 条件: `fn eval_binary(lhs: Token, op: Token, rhs: Token) -> Result<i32, String>` を実装する

### 34. `parse_key_value`

- 主に定着: `Option` ではなく `Result` を使って失敗理由を分けること
- 課題: `fn parse_key_value(line: &str) -> Result<(&str, &str), &'static str>` を書く
- 条件: `=` がない、前半が空、後半が空を区別して `Err` にする

## 5. モジュール・パス・ファイルI/O

### 35. `math.rs`

- 主に定着: `mod` と `use` の最小構成
- 課題: `math.rs` に `pub fn clamp(n: i32, min: i32, max: i32) -> i32` を書き、`main.rs` から呼ぶ
- 条件: `use crate::math::clamp;` を使う

### 36. `text.rs`

- 主に定着: 1 つの module に複数の関数を置いて使うこと
- 課題: `text.rs` に `first_word` と `word_count` を実装する
- 条件: `main.rs` 側では `use crate::text::{first_word, word_count};` を使う

### 37. `count_nonempty_lines`

- 主に定着: `std::fs::read_to_string` と `std::io::Error`
- 課題: `fn count_nonempty_lines(path: &str) -> Result<usize, std::io::Error>` を書く
- 条件: 空行と空白だけの行は数えない

### 38. `read_numbers`

- 主に定着: I/O と parse 失敗を自分の `String` エラーにまとめること
- 課題: `fn read_numbers(path: &str) -> Result<Vec<i32>, String>` を書く
- 条件: 1 行 1 整数。失敗時は「何行目で失敗したか」をエラーメッセージに入れる

### 39. `sum_file`

- 主に定着: 補助関数の `Result` を `?` でつなぐこと
- 課題: `fn sum_file(path: &str) -> Result<i32, String>` を書く
- 条件: 38 の `read_numbers` を呼んで合計する

### 40. `packet` module

- 主に定着: 長めの module path を `use` で短くすること
- 課題: `packet/types.rs` に `enum Packet`、`packet/parse.rs` に `fn parse_packet(line: &str) -> Result<Packet, String>` を置く
- 条件: `main.rs` では `use crate::packet::parse::parse_packet;` のように呼ぶ

## 6. `for`・イテレータ・ジェネリクス・ライフタイム

### 41. 3つの `for`

- 主に定着: `for x in &v`, `for x in &mut v`, `for x in v` の違い
- 課題: 次の 3 関数を書く
- 条件: `fn sum_readonly(v: &Vec<i32>) -> i32`, `fn double_in_place(v: &mut Vec<i32>)`, `fn consume_and_sum(v: Vec<i32>) -> i32`

### 42. `indexed_sum`

- 主に定着: `enumerate()` とパターン分解
- 課題: `fn indexed_sum(xs: &[i32]) -> i32` を書く
- 条件: `index * value` の総和を返す

### 43. `longest`

- 主に定着: ライフタイム引数が「どちらが長く生きるか」ではなく「返り値がどちら由来か」を示すこと
- 課題: `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` を書く
- 条件: 長い方を返す。長さが同じならどちらでもよい

### 44. `largest`

- 主に定着: ジェネリクスと trait bound の最小形
- 課題: `fn largest<T: Ord + Copy>(xs: &[T]) -> Option<T>` を書く
- 条件: 空なら `None`

### 45. `debug_join`

- 主に定着: `T: Debug` の意味と、generic 関数の制約
- 課題: `fn debug_join<T: std::fmt::Debug>(xs: &[T]) -> String` を書く
- 条件: 各要素を `{:?}` 形式で `", "` 区切りにして返す

### 46. `Bag` と `IntoIterator`

- 主に定着: `for x in &bag` が動くために何を実装するか
- 課題: `struct Bag { items: Vec<i32> }` を作り、`IntoIterator for &Bag` を実装する
- 条件: `for x in &bag` で各要素を読めるようにする。`type Item`, `type IntoIter` も自分で書く

## 7. `trait`・`where`・`trait object`・関連型

### 47. `Speak`

- 主に定着: `trait`、`impl Trait for Type`、`T: Trait` の基本形
- 課題: `trait Speak { fn speak(&self) -> String; }` を作り、`Dog`, `Cat` に実装する
- 条件: `fn chorus<T: Speak>(xs: &[T]) -> Vec<String>` を書く

### 48. `Box<dyn Speak>`

- 主に定着: `trait object` が「異なる具体型を同じ入れ物に入れる」ための道具だという感覚
- 課題: 47 の `Speak` を使って `fn chorus_dyn(xs: &[Box<dyn Speak>]) -> Vec<String>` を書く
- 条件: `Dog` と `Cat` を同じ `Vec<Box<dyn Speak>>` に入れて試す

### 49. `Summary` と `where`

- 主に定着: 条件が長くなったときに `where` へ逃がす書き方
- 課題: `trait Summary { fn summary(&self) -> String; }` を作り、`Article` と `ErrorLog` に実装する
- 条件: `fn dump_summaries<T>(xs: &[T]) -> Vec<String> where T: Summary + std::fmt::Debug` を書く

### 50. `Add<Output = T>`

- 主に定着: 関連型 `Output` と `impl Add for Point` の読み方
- 課題: `struct Point { x: i32, y: i32 }` に `std::ops::Add` を実装する
- 条件: `type Output = Point` を書き、さらに `fn add_three<T>(a: T, b: T, c: T) -> T where T: std::ops::Add<Output = T>` を実装する

## 補足

- まずは 01 から 34 までを、`for` / `match` / `impl` を素直に使って解くとよいです。
- 46 以降は少し重いので、詰まったら `knowledge/impl_lifetimes_and_associated_types.md` と `knowledge/trait_struct_and_trait_object.md` を見返すと進めやすいです。
- 50 問を一気に解くより、10 問ごとに「何が自然に書けるようになったか」を言語化した方が定着しやすいです。
