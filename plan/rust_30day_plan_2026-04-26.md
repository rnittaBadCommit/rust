# Rust 30日学習計画

## 前提

- C言語はスムーズに読み書きできる。
- C++はテンプレート、`vector`、`map`、クラスの概念を理解している。
- RAII、ムーブ、参照、スマートポインタは重点的に補強する。
- Rust Book日本語版の1〜10章は、演習込みで理解済み。
- 1日あたり実働10時間。休憩時間はこの10時間に含めない。
- Linux前提で進める。
- async/networkingは深追いせず、CLI、同期I/O、プロセス、ファイル、低レイヤ読解を優先する。
- 30日後の主成果物は、小さな実用CLIを1つ作り込むこと。

## 休憩方針

- 毎日、原則としてH3後、H6後、H8後に休憩を入れる。
- 切りのよい箇所が近い場合は、少し前後してよい。
- 休憩は実働10時間には含めない。

## 忘却曲線への対応

- 毎日のH1は、前日だけでなく「1日前、3日前、7日前」の内容を短く再実装する。
- 読み直しではなく、何も見ずにコードを書く。
- 詰まった箇所は、その日のH10で短く記録し、翌日以降のH1で再演習する。

## 30日後の到達判定

- 90〜120分で小CLIを `cargo new` から作れる。
- `Result`, `Option`, iterator, trait bounds, lifetime errorを説明して直せる。
- `clap`, `anyhow`/`thiserror`, `serde`, `assert_cmd` を使える。
- 小さなcrateなら `main` から処理の流れを追える。
- `unsafe` ブロックを見て「何を保証しているか」を文章で説明できる。

## 教材の軸

- Rust Book残り章
- Command Line Applications in Rust: https://rust-cli.github.io/book/
- The Cargo Book: https://doc.rust-lang.org/cargo/
- The Rustonomicon: https://doc.rust-lang.org/nomicon/

## 日別計画

| 日 | H1 | H2 | H3 | H4 | H5 | H6 | H7 | H8 | H9 | H10 |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1-10章復習 | 所有権再実装 | move演習 | 借用演習 | `String/Vec` | `Result`復習 | 小CLI作成 | エラー処理 | テスト | 日次まとめ |
| 2 | Day1復習 | Book 11章 | unit test | integration test | `cargo test` | table test | CLIにtest追加 | clippy修正 | borrow error分析 | まとめ |
| 3 | Day1再実装 | Book 12章 | minigrep写経 | `env::args` | file I/O | `Result`整理 | minigrep改造 | 引数追加 | エラー表示 | まとめ |
| 4 | Day2復習 | iterator基礎 | `map/filter` | `collect` | `Result`+iter | `?`練習 | grep風CLI | iterator化 | test追加 | まとめ |
| 5 | Day3復習 | closure | `Fn/FnMut` | ownership比較 | sort/filter | trait導入 | 出力形式trait | refactor | clippy | まとめ |
| 6 | Day4復習 | `Option`深掘り | `Result`深掘り | custom error | `thiserror` | `anyhow` | CLI error設計 | exit code | test | まとめ |
| 7 | 週復習 | 何も見ずCLI | 詰まり記録 | 解き直し | Book 13章 | iterator演習 | 小課題3本 | code review | 修正 | 週まとめ |
| 8 | Day5復習 | smart pointer概観 | `Box` | `Rc` | `RefCell` | borrow動的検査 | tree構造 | 所有権比較 | 演習 | まとめ |
| 9 | Day6復習 | RAII/Drop | `Drop` | resource管理 | file wrapper | lifetime復習 | lifetime演習 | struct参照 | エラー分析 | まとめ |
| 10 | Day7復習 | trait bounds | generics | `where` | associated type | trait object | `dyn Trait` | CLI設計に適用 | test | まとめ |
| 11 | Day8復習 | module構成 | crate分割 | lib/bin構成 | public API | docs | CLIをlib化 | integration test | docs test | まとめ |
| 12 | Day9復習 | `clap` | derive parser | subcommand | config設計 | `PathBuf` | CLI骨格作成 | help整備 | test | まとめ |
| 13 | Day10復習 | `serde` | JSON/TOML | config load | error設計 | default値 | CLIに設定追加 | test fixture | clippy | まとめ |
| 14 | 週復習 | 既存CLI模写 | `wc`実装 | `head`実装 | `find`簡易版 | 共通化 | test | benchmark軽く | 修正 | 週まとめ |
| 15 | Day11復習 | Linux file I/O | metadata | permissions | symlink | directory walk | `walkdir` | CLIに探索追加 | test | まとめ |
| 16 | Day12復習 | process | `Command` | pipe | exit status | signal概観 | 子プロセスCLI | error処理 | test | まとめ |
| 17 | Day13復習 | stdin/stdout | buffering | `BufRead` | streaming処理 | 大入力対応 | grep改良 | memory確認 | test | まとめ |
| 18 | Day14復習 | ownership総復習 | lifetime総復習 | trait総復習 | iterator総復習 | weak点演習 | 自力実装 | review | 修正 | まとめ |
| 19 | Day15復習 | OSS読解準備 | `cargo tree` | `cargo doc` | crate構造読む | 小crate読む | call graph作成 | 改造案 | 小改造 | まとめ |
| 20 | Day16復習 | `fd`/小CLI読解 | main追跡 | clap追跡 | error追跡 | iterator追跡 | 改造 | test | 差分整理 | まとめ |
| 21 | 週復習 | 自作CLI設計 | 要件定義 | crate構成 | error設計 | test設計 | 実装開始 | parser | file処理 | 週まとめ |
| 22 | Day18復習 | 自作CLI中核 | iterator化 | streaming | error改善 | subcommand | test追加 | refactor | docs | まとめ |
| 23 | Day19復習 | Linux低レイヤ | fd概念 | `std::os::unix` | permissions | `/proc` | proc読むCLI | test | 整理 | まとめ |
| 24 | Day20復習 | unsafe概観 | raw pointer | unsafe boundary | soundness | Rustonomicon抜粋 | unsafe読解 | safe wrapper | メモ | まとめ |
| 25 | Day21復習 | `Vec`内部概念 | allocation | pointer | ZST概念 | drop glue | unsafeコード読解 | 契約を書く | メモ | まとめ |
| 26 | Day22復習 | concurrency基礎 | thread | channel | Mutex | Arc | 並列探索CLI | test | race観点 | まとめ |
| 27 | Day23復習 | OSS読解2 | `ripgrep`一部 | main/config | search path | error path | 読解メモ | 小改造案 | 模倣実装 | まとめ |
| 28 | 週復習 | 自作CLI仕上げ | edge case | help改善 | test強化 | clippy | README | release build | 手動確認 | 週まとめ |
| 29 | 総復習1 | 所有権問題集 | lifetime問題 | trait問題 | iterator問題 | error問題 | 何も見ず小CLI | 比較修正 | 弱点補強 | まとめ |
| 30 | 総復習2 | 最終CLI作成 | parser | 中核処理 | error/test | refactor | OSS読解試験 | unsafe読解試験 | 到達判定 | 次計画 |
