# Rust Practice Workspace

このディレクトリは、[`rust_practice_50_questions.md`](../../rust_practice_50_questions.md) に対応する演習用 workspace です。

各課題は 1 crate ずつに分けています。
まずは 00 から順に、`src/lib.rs` の `todo!()` を埋めて進める想定です。

## 進め方

1. 解きたい crate の `src/lib.rs` を読む
2. コメントを見ながら関数宣言と実装を自分で書く
3. `cargo test -p <crate名>` でその課題だけ確認する
4. 詰まったら [`rust_practice_50_questions.md`](../../rust_practice_50_questions.md) と `knowledge/` を見直す

個別確認例:

```bash
cd /home/rnitta/rust_study/ex/ex04_practice
cargo test -p ex04_borrowed_len
cargo test -p ex04_first_word
```

## 全体確認

```bash
cd /home/rnitta/rust_study/ex/ex04_practice
cargo test --workspace --no-run
```

## 課題一覧

### 00_borrowed_len

対応:

- 問題01: `borrowed_len`

目的:

- 読むだけの処理では `&str` を使う
- 所有権を受け取らなくてもよい場面を見分ける

### 01_append_exclamation

対応:

- 問題02: `append_exclamation`

目的:

- `&mut String` でその場更新する
- 所有権を取らずに書き換える感覚に慣れる

### 02_take_and_report

対応:

- 問題03: `take_and_report`

目的:

- 値で受け取ると所有権が移ることを確認する
- 受け取った値を戻り値で返す形に慣れる

### 03_longer_len

対応:

- 問題04: `longer_len`

目的:

- 複数の共有借用を同時に扱う
- 読むだけなら複数借用できることを確認する

### 04_count_positive

対応:

- 問題05: `count_positive`

目的:

- `&[i32]` を読む関数に慣れる
- `Vec` ではなく slice を受ける感覚を付ける

### 05_negate_all

対応:

- 問題06: `negate_all`

目的:

- `&mut [i32]` をその場で更新する
- 可変 slice を回して値を書き換える

### 06_take_last

対応:

- 問題07: `take_last`

目的:

- `Vec` から値を安全に取り出す
- 空の可能性を `Option` で表す

### 07_duplicate_last

対応:

- 問題08: `duplicate_last`

目的:

- 値を読む操作と push の順序に慣れる
- 借用と更新の順番を意識する

### 08_first_word

対応:

- 問題09: `first_word`

目的:

- `&str` から部分文字列を借用のまま返す
- 新しい `String` を作らずに解く

### 09_split_once_colon

対応:

- 問題10: `split_once_colon`

目的:

- 1 個の `&str` から 2 個の `&str` を切り出す
- `Option<(&str, &str)>` に慣れる

## 補足

- どの crate も、関数宣言はまだ書いていません。
- テストは通常のコードとして入っています。
- そのため、関数宣言を書いていない段階では `cargo test` は通りません。
