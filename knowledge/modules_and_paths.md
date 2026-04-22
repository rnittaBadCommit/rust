# モジュール、パス、公開 API

このノートは、module、path、`use`、`pub`、`crate::`、`super::`、再エクスポートをまとめます。

## module の役割

module はコードの名前空間とプライバシー境界を作ります。

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
```

module tree は crate root から始まります。

- binary crate の root: `src/main.rs`
- library crate の root: `src/lib.rs`

## path

`std::io::Error` のような名前は path です。

```rust
std::io::Error
```

これは:

- `std`: 標準ライブラリ crate
- `io`: その中の module
- `Error`: その中の型

`std::slice::Iter<'a, i32>` のような長い名前も、同じく「どの module の中の何か」を表しています。

## `crate::`

`crate::foo::bar` の `crate` は、今コンパイルしている crate の root を指します。

```rust
use crate::util::parse;
```

これは「同じ crate の中にある `util::parse` を使う」です。

workspace 全体を指すわけではありません。別 crate を使うには `Cargo.toml` に依存を宣言し、その crate 名から path を書きます。

## 絶対 path と相対 path

crate root から始める:

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

今いる module から相対的に始める:

```rust
front_of_house::hosting::add_to_waitlist();
```

親 module へ戻るには `super::` を使います。

```rust
super::serve_order();
```

## privacy と `pub`

Rust の item はデフォルト private です。

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

`pub` を付けると外から見えるようになります。

注意点:

- `pub struct` でもフィールドはデフォルト private
- 公開したいフィールドには個別に `pub` が必要
- `pub enum` の variant はまとめて公開される

```rust
pub struct User {
    pub name: String,
    age: u32,
}
```

この例では `name` は外から見えますが、`age` は見えません。

## `use`

`use` は長い path に短い名前を付けるものです。

```rust
use std::io;

fn f() -> io::Result<()> {
    Ok(())
}
```

`use` は C の `#include` とは違います。ファイルを貼り付けるのではなく、名前をスコープに持ち込みます。

trait 名でも同じです。

```rust
use std::fmt::Debug;

fn print_debug<T: Debug>(x: T) {
    println!("{x:?}");
}
```

## `use` のまとめ書き

nested path:

```rust
use std::{cmp::Ordering, io};
```

module 自体と中の item を同時に持ち込む:

```rust
use std::io::{self, Write};
```

glob:

```rust
use std::collections::*;
```

glob はテストや prelude 的用途では便利ですが、通常は何が入るか見えにくいので慎重に使います。

## 再エクスポート `pub use`

`pub use` は、内部構造と公開 API の見せ方を分けるために使います。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;
```

内部では深い module に置いていても、外からは短い path で使えるようにできます。

## ファイル分割

`mod garden;` と書くと、Rust は対応するファイルを探します。

```rust
mod garden;
```

対応例:

- `src/garden.rs`
- `src/garden/mod.rs`

さらに子 module を置く場合:

```text
src/
  main.rs
  garden.rs
  garden/
    vegetables.rs
```

`garden.rs` 側:

```rust
pub mod vegetables;
```

## package, crate, module の切り分け

- package: Cargo の管理単位。`Cargo.toml` で見る
- crate: Rust のコンパイル単位。`main.rs` / `lib.rs` で見る
- module: crate 内の名前空間と privacy 境界

C と比べると、module は header/source の分割そのものではありません。Rust の module は、名前空間と公開範囲を制御する仕組みです。
