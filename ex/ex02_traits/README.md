# Trait Practice

このディレクトリは、trait の理解に絞った練習用ワークスペースです。

## Exercises

### 00_animal

目的:

- `trait` を自分で定義する
- `impl Trait for Type` を書く
- `T: Trait` の形の関数を読む

やること:

- `Animal` trait を読む
- `Dog` と `Cat` に `Animal` を実装する
- `speak_once<T: Animal>` を実装する

### 01_display_point

目的:

- 標準ライブラリの trait を自分で実装する
- `impl Display for Point` を書く
- `T: Display` の関数を書く

やること:

- `Point` に `Display` を実装する
- `render<T: Display>` を実装する

## How To Run

```bash
cd /home/rnitta/rust_study/diary/ex02_traits
cargo test -p ex02_animal
cargo test -p ex02_display_point
```

全体確認:

```bash
cargo test --workspace --no-run
```

## Reading Order

1. [[../topics/traits_basics]]
2. `00_animal`
3. `01_display_point`

## Note

`todo!()` のままでもコンパイル自体は通ります。
テストを通すには、自分で中身を埋めてください。
