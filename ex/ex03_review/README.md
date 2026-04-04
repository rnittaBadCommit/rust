# Rust Review Workspace

このディレクトリは、[`rust_review_questions.md`](../../rust_review_questions.md) に対応する復習用ワークスペースです。
`trait` を除いた基礎項目を、1問ずつ小さなクレートで解けるようにしています。

## 進め方

1. 解きたい課題の `src/lib.rs` を読む
2. `todo!()` になっている関数やメソッドを自分で実装する
3. `cargo test -p <crate名>` でその課題だけ確認する
4. 詰まったら [`rust_review_hints.md`](../../rust_review_hints.md) を見る

全体確認:

```bash
cd /home/rnitta/rust_study/ex/ex03_review
cargo test --workspace --no-run
```

個別確認例:

```bash
cargo test -p ex03_split_key_value
cargo test -p ex03_nonempty_line_count
```

## 課題一覧

### 00_split_key_value

対応:

- 問題1: `&str` とスライス

目的:

- `&str` を借用のまま分割する
- `Option<T>` で失敗を表す
- 新しい `String` を作らずにスライスを返す

### 01_clamp_scores

対応:

- 問題2: 可変借用と `Vec`

目的:

- `&mut Vec<i32>` を受け取ってその場で更新する
- `for x in &mut v` や `iter_mut()` に慣れる

### 02_stack

対応:

- 問題3: `struct` と `impl`

目的:

- `Vec<i32>` を内部に持つ型を作る
- `&self` と `&mut self` の使い分けに慣れる
- `Option<i32>` と `Option<&i32>` の違いを確認する

### 03_packet

対応:

- 問題4: `enum` と `match`

目的:

- 列挙型の各バリアントを `match` で処理する
- 借用した値から説明文字列を作る

### 04_nonempty_line_count

対応:

- 問題5: `Result` とファイルI/O

目的:

- `std::fs::read_to_string` と `?` に慣れる
- 空行と空白行を除外して数える

## 補足

- どの課題も、まずはテストを通すことを目標にしています。
- `02_stack` と `04_nonempty_line_count` には、問題文に合わせた `src/main.rs` も入れてあります。
- 既存の `ex/ex01` や `ex/ex02_traits` と同じく、解答は置かず `todo!()` から埋める形です。

