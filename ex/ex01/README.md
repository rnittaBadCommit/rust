# Rust演習セット: C経験者向け

このディレクトリは、[[rust_from_c_guide]] を読んだあとに手を動かすための演習セットです。
小さな課題を順番に実装しながら、Rustの所有権、借用、`Option`、`Result`、`enum` に慣れる構成にしています。

## 進め方

1. まず各課題の `src/lib.rs` を読む
2. `todo!()` になっている関数を自分で実装する
3. `cargo test -p <crate名>` でその課題だけ確認する
4. 詰まったら、コンパイラエラーと `rust_from_c_guide.md` を見直す

全体の確認:

```bash
cd /home/rnitta/rust_study/diary/ex01
cargo test --workspace --no-run
```

個別の確認例:

```bash
cargo test -p ex00_strlen
cargo test -p ex01_max_option
```

## 課題一覧

### 00_strlen

目的:

- `&str` に慣れる
- Cの `strlen` と Rustの `str::len()` の違いを意識する

ポイント:

- Cの `strlen` はNUL終端までのバイト数
- Rustの `&str` はUTF-8文字列スライス
- `s.len()` は文字数ではなくバイト数

### 01_max_option

目的:

- 空配列を安全に扱う
- `Option<T>` で「値がない」を表現する

Cとの比較:

- Cでは `-1` のような番兵値や出力引数を使いがち
- Rustでは `Option<i32>` で欠如を明示する

### 02_stack

目的:

- `Vec<T>` を使ってスタックを実装する
- `Option<&T>` と `Option<T>` の違いを体感する

Cとの比較:

- Cなら自前で容量や再確保を管理することが多い
- Rustでは `Vec<T>` に任せ、操作の安全性に集中できる

### 03_line_count

目的:

- ファイルI/Oと `Result<T, E>` に慣れる
- `?` を使ってエラーを自然に伝播する

Cとの比較:

- Cでは戻り値と `errno` を自前で確認する
- Rustでは `Result` を戻り値に乗せる

### 04_value_enum

目的:

- Cのタグ付きunionをRustの `enum` で置き換える感覚を掴む
- `match` の網羅性チェックに慣れる

## 取り組み順

1. `00_strlen`
2. `01_max_option`
3. `02_stack`
4. `03_line_count`
5. `04_value_enum`

## 進めるときの実用メモ

- 関数引数は、所有権が不要ならまず `&T` を考える
- 空の可能性があるなら `Option<T>` を考える
- 失敗し得る処理なら `Result<T, E>` を考える
- `String` を受け取る前に、本当に所有が必要か確認する
- 迷ったらまず `cargo check` や `cargo test` を回す

## 補足

この演習セットは、最初から答えを置かずに「コンパイラとテストに導かれて学ぶ」形にしています。
必要なら次に、各課題の模範解答と解説も別ファイルで追加できます。
