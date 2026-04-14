# Rust Practice Workspace 2

このディレクトリは、[`rust_practice_50_questions.md`](../../rust_practice_50_questions.md) の問題 11〜20 に対応する演習用 workspace です。
`ex04_practice` の続きとして、crate 名は 10〜19 の連番でそろえています。

各課題は 1 crate ずつに分けています。
まずは `src/lib.rs` のコメントを見ながら、関数宣言や `struct` / `impl` を自分で書いて進める想定です。

## 進め方

1. 解きたい crate の `src/lib.rs` を読む
2. コメントを見ながら関数、`struct`、`impl` を自分で書く
3. `cargo test -p <crate名>` でその課題だけ確認する
4. 詰まったら [`rust_practice_50_questions.md`](../../rust_practice_50_questions.md) と `knowledge/` を見直す

個別確認例:

```bash
cd /home/rnitta/rust_study/ex/ex05_practice
cargo test -p ex05_middle_two
cargo test -p ex05_bank_account
```

## 全体確認

```bash
cd /home/rnitta/rust_study/ex/ex05_practice
cargo test --workspace --no-run
```

## 課題一覧

### 10_middle_two

対応:

- 問題11: `middle_two`

目的:

- `[T; N]` と `&[T]` の違いに慣れる
- 配列から slice を切り出して返す感覚を付ける

### 11_max_slice

対応:

- 問題12: `max_slice`

目的:

- 空 slice の可能性を `Option` で表す
- `&[i32]` を読む処理に慣れる

### 12_reverse_in_place

対応:

- 問題13: `reverse_in_place`

目的:

- `&mut [i32]` をその場で更新する
- 新しい `Vec` を作らずに並びを入れ替える

### 13_collect_lengths

対応:

- 問題14: `collect_lengths`

目的:

- `&[String]` を読むだけで処理する
- 所有権を奪わずに必要な情報だけ集める

### 14_join_nonempty

対応:

- 問題15: `join_nonempty`

目的:

- `&str` の列から `String` を組み立てる
- 空文字列を飛ばしながら区切り文字を扱う

### 15_drop_empty

対応:

- 問題16: `drop_empty`

目的:

- `Vec<String>` をその場で絞り込む
- 保持する要素と捨てる要素を条件で分ける

### 16_counter

対応:

- 問題17: `Counter`

目的:

- `struct` の内部状態を `impl` のメソッドで更新する
- `&self` と `&mut self` の違いに慣れる

### 17_rectangle

対応:

- 問題18: `Rectangle`

目的:

- 読み取り専用メソッドを `&self` で書く
- フィールドから計算するメソッドを分けて考える

### 18_bank_account

対応:

- 問題19: `BankAccount`

目的:

- `&mut self` と `Result` を組み合わせる
- 正常系と失敗系を同じ API で表す

### 19_point

対応:

- 問題20: `Point`

目的:

- `&mut self` で座標を更新する
- `&self` で読み取り計算する

## 補足

- 関数宣言や `impl` のメソッド宣言も、最初はまだ書いていません。
- テストは通常のコードとして入っています。
- そのため、宣言を書いていない段階では `cargo test` は通りません。
