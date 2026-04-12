# AGENTS.md

## Purpose

このリポジトリは Rust 学習用です。利用者は C、アセンブリ、Linux、スタック/ヒープの基礎をすでに理解している前提で扱ってください。

## Communication

- 既定では日本語で説明する
- Rust の概念説明では、必要に応じて C と比較する
- Rust の概念説明では、まず C で説明や例えができないかを考える
- C だと不自然、回りくどい、対応物が弱い場合は C++ と比較する
- C++ の方が簡潔で対応が直接的なら、C++ で説明してよい
- 抽象論よりも、小さく実行できる例と直接的な説明を優先する

## Startup Read

- このディレクトリで Codex を起動したら、最初に `AGENTS.md` に加えて `.codex/` 配下と `knowledge/` 配下の内容を読む
- `.codex/` 配下は、このリポジトリでの運用ルールと補助メモとして扱う
- `knowledge/` 配下は、利用者の現在の Rust に関する知識として扱う
- 起動時に読みやすいよう、`.codex/` と `knowledge/` の README や索引は必要に応じて更新する

## Repository Map

- `rust_from_c_guide.md`: C 経験者向けの主教材
- `.codex/`: Codex が最初に読む運用ルールと補助メモ
- `knowledge/`: 利用者の現在の Rust 知識を整理して蓄積する知識ベース
- `diary/`: Obsidian 形式の学習ノート、テンプレート、進捗管理
- `ex/`: 手を動かすための演習集
  - `ex/ex01`, `ex/ex02_traits`, `ex/ex03_review` はそれぞれ独立した Cargo workspace
  - リポジトリ直下に Cargo workspace はない前提で扱う

## Knowledge Management

- `knowledge/` 配下の内容は、利用者の現在の Rust に関する知識として扱う
- Codex への質問、コードレビュー、説明、修正を通して再利用価値のある新しい知識や考え方が出てきたら、`knowledge/` 配下へ整理して追加する
- `knowledge/` 配下の情報は、読み返しやすさを優先してファイルを適切に分割・統合する
- 整理しやすい場合は、`knowledge/` 配下にサブディレクトリを作ってよい
- 「習いはしたが、まだしっかり身についていない知識」は `knowledge/not_yet_internalized/` に保存する
- ただし `knowledge/not_yet_internalized/` への保存は、利用者が明示した場合に限る
- 利用者の要望のうち、その場限りではなく次回以降も有効そうなものがあれば、自動読込される Codex 設定へ追加するかを提案する

## Working Rules

- 変更は小さく、学習価値が高い形に保つ
- 作業の節目では `git status` を確認し、差分を把握しながら進める
- Git でのバージョン管理を前提とし、無関係な変更を混ぜずに意味のある単位で履歴を残す
- Codex が書いた変更だけでなく、ユーザーが追加・更新した関連ファイルも必要に応じてコミット対象に含めてよい
- `/home/rnitta/rust_study` を作業ルートとして扱い、この配下のファイルは必要なら追加、編集、削除してよい
- `/home/rnitta/rust_study` 配下の作業では、編集前の確認は不要
- このルート配下以外のファイルを編集するときは、意図を説明したうえで毎回確認を取る
- `diary/` の再編成やノート名変更は、依頼がない限り行わない
- `diary/` の Obsidian wiki link を壊さない
- `diary/` にノートを追加するときは `./.codex/vault_conventions.md` に従う
- 演習を変更したときは、対象 workspace または crate で `cargo test` か `cargo check` を実行する
- 単なる最短修正と、理解を助ける修正の両方があるなら、後者を優先する
- レビュー時は `./.codex/review_checklist_template.md` を叩き台にする
- レビューでは一般的なアプリケーション向けの観点に加えて、kernel、ランタイム、何度も呼ばれるライブラリ、ホットパスでは高性能要求コード向けの観点でも確認する
- 演習コードのレビューでは、まず手で書いた実装の改善点を述べる
- 演習コードのレビューでは、標準ライブラリや既存メソッドでより自然に書ける版がまだ出ていない場合、その書き方も具体例つきで紹介する
- 演習コードのレビューで、手書き版と標準ライブラリ版の両方がある場合は、学習用としての意義と実用上どちらが自然かの両方を整理して述べる

## Supporting Notes

- `./.codex/README.md`
- `./.codex/repo_workflow.md`
- `./.codex/review_checklist_template.md`
- `./.codex/vault_conventions.md`
