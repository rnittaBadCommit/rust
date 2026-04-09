# Knowledge

このディレクトリは、利用者の現在の Rust に関する知識を蓄積するための場所です。

## Index

- `array_and_slice.md`: 配列 `[T; N]` とスライス `[T]` / `&[T]` の違い。`&[T]` を C の `T* + len` と対比して整理
- `attributes_and_derive.md`: `#[derive(Debug)]`、属性 `#[...]`、`Copy` / `Clone` をどう読むか
- `for_and_intoiterator.md`: `for x in xs`, `for x in &xs`, `for x in xs.iter()` の違いと、`IntoIterator` / `Iterator` / `iter()` の役割分担
- `impl_lifetimes_and_associated_types.md`: `impl<'a>`, `type IntoIter = ...`, `Self::IntoIter` の読み方
- `module_paths_and_use.md`: `std::io::Error` のようなパスと `use` の役割
- `patterns_and_while_let.md`: `Some(v)`, `Ok(x)`, `(i, x)`, `while let` の共通する読み方
- `rust_from_c_guide.md`: C 経験者向けの主教材
- `trait_struct_and_trait_object.md`: `trait` と `struct` の役割の違い、`trait bound` と `trait object` の使い分け
- `where_clause.md`: `where` 句の役割と trait bound の書き場所の違い
- `not_yet_internalized/trait_basics.md`: trait の基礎整理。`trait`, `trait bound`, `impl Trait for Type`, `Add<Output = T>` を分解したノート

## Rule

- 質問、レビュー、コード修正から得られた再利用価値のある知識を残す
- 1 ファイル 1 テーマを基本にする
- 整理しやすい場合は、サブディレクトリを作ってよい
- 後から読み返しやすいように、必要に応じて既存ファイルを分割・統合する
- まだ十分に身についていない知識は、明示指示がある場合のみ `not_yet_internalized/` に保存する
