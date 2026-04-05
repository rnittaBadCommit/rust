# .codex

このディレクトリは、この Rust 学習リポジトリで使う Codex 向けの補助メモ置き場です。

## Startup

- このディレクトリで Codex を起動したら、`AGENTS.md` に加えて `.codex/` 配下の内容を最初に読む
- `.codex/` は運用ルール、`knowledge/` は利用者の知識ベースとして扱う
- 起動時に読みやすい状態を保つため、必要なら README や索引を更新する

## Files

- `README.md`: `.codex/` 配下の入口
- `repo_workflow.md`: リポジトリ構成、学習前提、テスト方針
- `review_checklist_template.md`: レビュー時の叩き台
- `vault_conventions.md`: `diary/` 配下のノート命名規則と更新方針

## Note

- 学習コンテンツ本体は `rust_from_c_guide.md`, `diary/`, `ex/` に置く
- `.codex/` には運用メモだけを置く
