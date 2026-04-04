# Repo Workflow

## Purpose

- このリポジトリは、C/Linux の基礎がある人が Rust を学ぶための作業場所
- 所有権、借用、`Option`, `Result`, trait を C と対比しながら理解する方針を優先する

## Main Materials

- `rust_from_c_guide.md`: 主教材
- `rust_review_hints.md`: 復習用ヒント
- `rust_review_questions.md`: 復習用の問い
- `diary/`: 学習日記、トピック整理、コード断片、テンプレート
- `ex/`: 演習と確認用 workspace

## Exercise Layout

- `ex/ex00`: 小さな単発サンプル
- `ex/ex01`: 基本文法と所有権まわりの演習 workspace
- `ex/ex02_traits`: trait 集中演習 workspace
- `ex/ex03_review`: 復習用演習 workspace

## Command Rule

- リポジトリ直下には Cargo workspace がない
- `cargo` コマンドは対象の crate または workspace に移動して実行する
- 例:
  - `cd ex/ex01 && cargo test --workspace --no-run`
  - `cd ex/ex02_traits && cargo test -p ex02_animal`
  - `cd ex/ex03_review && cargo test --workspace --no-run`

## Editing Preference

- 学習用リポジトリとしての読みやすさを保つ
- ひねった抽象化より、最小の実例、テスト、短い説明を優先する
- 依頼がない限り、ノート配置の大規模変更や無関係な整形はしない
- Git の差分を見ながら進め、関連する変更だけをまとめる
- 履歴は小さく意味のある単位に保つ
