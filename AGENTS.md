# AGENTS.md

## Purpose

このリポジトリは Rust 学習用です。利用者は C、アセンブリ、Linux、スタック/ヒープの基礎をすでに理解している前提で扱ってください。

## Communication

- 既定では日本語で説明する
- Rust の概念説明では、必要に応じて C と比較する
- 抽象論よりも、小さく実行できる例と直接的な説明を優先する

## Repository Map

- `rust_from_c_guide.md`: C 経験者向けの主教材
- `diary/`: Obsidian 形式の学習ノート、テンプレート、進捗管理
- `ex/`: 手を動かすための演習集
  - `ex/ex01`, `ex/ex02_traits`, `ex/ex03_review` はそれぞれ独立した Cargo workspace
  - リポジトリ直下に Cargo workspace はない前提で扱う

## Working Rules

- 変更は小さく、学習価値が高い形に保つ
- 作業の節目では `git status` を確認し、差分を把握しながら進める
- Git でのバージョン管理を前提とし、無関係な変更を混ぜずに意味のある単位で履歴を残す
- Codex が書いた変更だけでなく、ユーザーが追加・更新した関連ファイルも必要に応じてコミット対象に含めてよい
- `diary/` の再編成やノート名変更は、依頼がない限り行わない
- `diary/` の Obsidian wiki link を壊さない
- `diary/` にノートを追加するときは `./.codex/vault_conventions.md` に従う
- 演習を変更したときは、対象 workspace または crate で `cargo test` か `cargo check` を実行する
- 単なる最短修正と、理解を助ける修正の両方があるなら、後者を優先する

## Supporting Notes

- `./.codex/repo_workflow.md`
- `./.codex/vault_conventions.md`
