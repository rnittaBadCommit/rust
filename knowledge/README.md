# Knowledge

このディレクトリは、利用者の現在の Rust に関する知識を蓄積するための場所です。

## Index

- `foundations_and_cargo.md`: Rust の全体像、`rustc` / `cargo`、crate / package / workspace、変数、基本型、式、関数、制御構文、学習順序
- `ownership_memory_and_strings.md`: 所有権、ムーブ、借用、`Copy` / `Clone`、配列、slice、`String` / `&str`、UTF-8、`char_indices()`、`Vec` / `Box` のメモリ像、lifetime の基礎
- `data_patterns_and_errors.md`: `struct`、`impl`、`self` / `Self`、属性と `derive`、`enum`、`match`、pattern、`Option<T>`、`Result<T, E>`、`?`、`unwrap` / `expect`、panic
- `modules_and_paths.md`: module、path、`crate::`、`super::`、privacy、`pub`、`use`、`pub use`、ファイル分割
- `traits_generics_and_iterators.md`: generics、trait、trait bound、`where`、関連型、`impl Trait`、trait object、`Sized` / `?Sized`、`for` / `IntoIterator`、iterator、closure
- `the_book_early_topics.md`: The Rust Programming Language 日本語版の 1〜6章から、項目別ノートへの対応と補足だけを残した索引
- `not_yet_internalized/i32_vector_raw_parts.md`: `Vec<i32>` 風コンテナを自作するときの `ptr` / `len` / `cap`、`NonNull`、`Layout`、allocator API、`Index` / `IndexMut`、safe API に閉じ込める `unsafe`
- `not_yet_internalized/trait_basics.md`: trait の基礎整理。`trait`, `trait bound`, `impl Trait for Type`, `Add<Output = T>` を分解したノート
- `not_yet_internalized/the_book_ch07_onward.md`: The Rust Programming Language 日本語版の 7章以降について、まだ曖昧な知識として章別に整理

## Rule

- 質問、レビュー、コード修正から得られた再利用価値のある知識を残す
- 1 ファイル 1 テーマを基本にする
- 整理しやすい場合は、サブディレクトリを作ってよい
- 後から読み返しやすいように、必要に応じて既存ファイルを分割・統合する
- まだ十分に身についていない知識は、明示指示がある場合のみ `not_yet_internalized/` に保存する
