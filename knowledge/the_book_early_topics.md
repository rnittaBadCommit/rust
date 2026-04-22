# The Book 1〜6章の補足

対象: https://doc.rust-jp.rs/book-ja/

このノートは、The Rust Programming Language 日本語版の 1〜6章から、既存の項目別ノートに吸収しきれない補足だけを残す索引です。

7章以降は、まだ曖昧な知識として `not_yet_internalized/the_book_ch07_onward.md` にあります。

## 1章: 事始め

1章の内容は主に `foundations_and_cargo.md` に統合しています。

追加で意識する点:

- `rustup` は Rust 本体、標準ライブラリ、Cargo、rustdoc などをまとめて入れる管理ツール。
- `target/debug` と `target/release` はビルド成果物の置き場。
- `cargo check` は実行ファイル生成を省くので、編集途中の確認に向いている。

## 2章: 数当てゲーム

2章は、所有権より先に `Result`, `match`, 外部 crate, loop を浅く全部見せる章です。

この段階では全部を理解し切るより、コンパイラが型と失敗を追跡している感覚を掴むのが目的です。

## 3章: 一般的なプログラミング概念

3章の内容は `foundations_and_cargo.md` に統合しています。

特に重要な補足:

- `let` はデフォルト不変。
- シャドーイングは再束縛なので、型を変えられる。
- `if` や `loop` は式として値を返せる。
- 配列添字が範囲外なら panic。C の未定義動作とは違う。

## 4章: 所有権

4章の内容は `ownership_memory_and_strings.md` に統合しています。

覚える規則:

- 各値には所有者が 1 つある。
- 所有者がスコープを抜けると値は破棄される。
- 同時に所有者は 1 つだけ。
- 共有参照は複数 OK。
- 可変参照は同時に 1 つだけ。

## 5章: 構造体

5章の内容は `data_patterns_and_errors.md` に統合しています。

特に読み返す点:

- フィールド初期化省略記法
- 構造体更新記法
- tuple struct
- unit-like struct
- `#[derive(Debug)]`
- `dbg!`
- 関連関数とメソッド

## 6章: enum と pattern matching

6章の内容は `data_patterns_and_errors.md` に統合しています。

Rust の enum は、C の tagged union に近い構造を型安全に書ける仕組みです。

`match` は網羅的でなければならないため、「処理漏れ」をコンパイラに見つけてもらえます。
