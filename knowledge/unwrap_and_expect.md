# `unwrap` と `expect`

このノートは、`Option<T>` や `Result<T, E>` から中身を取り出す
`unwrap()` と `expect()` を整理するためのものです。

## まず一言で

- `unwrap()` は「中身があるはず」と決め打ちで取り出す
- その前提が外れると `panic` で停止する
- `expect("...")` も同じだが、失敗時メッセージを自分で書ける
- 回復できる失敗を雑に `unwrap()` するより、まず `match` や `?` を考える

## C と比べると

感覚としては次に近いです。

- `Option<T>` の `unwrap()`: `NULL` ではない前提で強引に使う
- `Result<T, E>` の `unwrap()`: エラーならその場で `abort` するくらいの強さ
- `expect("reason")`: `assert(ptr != NULL && "reason")` に少し近い

ただし Rust は、
「失敗するかもしれない値」であることを
型で先に表している点が C よりかなり明示的です。

## `Option<T>` に対する `unwrap()`

`Option<T>` は:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

でした。

なので:

```rust
let x = Some(10);
let n = x.unwrap();
println!("{n}");
```

は `Some(10)` の中の `10` を取り出します。

一方で:

```rust
let x: Option<i32> = None;
let n = x.unwrap();
```

は `None` なので `panic` します。

感覚としては次を短く書いたものです。

```rust
let n = match x {
    Some(v) => v,
    None => panic!("called unwrap on a None value"),
};
```

## `Result<T, E>` に対する `unwrap()`

`Result<T, E>` は:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

でした。

なので:

```rust
let n: Result<i32, &str> = Ok(42);
let v = n.unwrap();
println!("{v}");
```

は `42` を取り出します。

一方で:

```rust
let n: Result<i32, &str> = Err("bad input");
let v = n.unwrap();
```

は `Err(...)` なので `panic` します。

感覚としては次に近いです。

```rust
let v = match n {
    Ok(v) => v,
    Err(e) => panic!("called unwrap on an Err value: {e:?}"),
};
```

## `expect()` は何が違うか

`expect()` も本質は同じです。
成功なら中身を取り出し、
失敗なら `panic` します。

違いは、失敗時メッセージを自分で書けることです。

```rust
let port: u16 = "8080"
    .parse()
    .expect("hard-coded port literal must be valid");
```

これは
「この文字列リテラルは固定値なので、失敗したらプログラム側のミス」
という意図をコードに残せます。

## `unwrap()` と `?` の違い

ここはかなり重要です。

```rust
let text = std::fs::read_to_string("hello.txt").unwrap();
```

これは、
ファイルが読めなければその場で `panic` して終了します。

一方:

```rust
let text = std::fs::read_to_string("hello.txt")?;
```

これは、
読めなければ `Err(...)` を呼び出し元へ返します。

つまり:

- `unwrap()`: 失敗したら止める
- `?`: 失敗を上へ返す

です。

CLI やライブラリでは、
外部入力やファイルI/Oの失敗は普通に起こりうるので、
`unwrap()` より `?` の方が自然なことが多いです。

## `unwrap_or()` と `unwrap_or_else()` の違い

この 2 つはどちらも、
失敗時に `panic` する代わりに代替値を返します。

違いは、代替値をいつ作るかです。

- `unwrap_or(x)`: `x` を先に評価する
- `unwrap_or_else(f)`: 必要なときだけ `f` を呼ぶ

## `Option<T>` での例

```rust
let a = Some(10);

let x = a.unwrap_or(20);
let y = a.unwrap_or_else(|| 20);
```

どちらも `Some(10)` なら `10` を返し、
`None` なら `20` を返します。

ただし評価のタイミングが違います。

```rust
let s = Some(String::from("hello"));

let x = s.clone().unwrap_or(String::from("fallback"));
let y = s.unwrap_or_else(|| String::from("fallback"));
```

このとき:

- `unwrap_or(...)` は `Some` でも `String::from("fallback")` を先に作る
- `unwrap_or_else(...)` は `None` のときだけ fallback を作る

なので、
代替値の作成が軽いなら `unwrap_or(...)`、
重いなら `unwrap_or_else(...)` が向いています。

## `Result<T, E>` での例

`Result` でも考え方は同じです。

```rust
let n: Result<i32, &str> = Ok(10);

let a = n.unwrap_or(0);
let b = n.unwrap_or_else(|_e| 0);
```

`Result` の `unwrap_or_else` では、
`Err(e)` の `e` を受け取れます。

```rust
let n: Result<i32, &str> = Err("bad input");

let v = n.unwrap_or_else(|e| {
    println!("error: {e}");
    0
});
```

`Option` には失敗理由がないので `|| ...` ですが、
`Result` では `|e| ...` と書けます。

## C と比べると

感覚としては次に近いです。

- `unwrap_or(x)`: fallback 値を先に用意しておく
- `unwrap_or_else(f)`: 失敗時だけ fallback 計算を呼ぶ

## どんなときに `unwrap()` を使ってよいか

次のような場面では比較的使いやすいです。

- テストコードで「失敗したらその場で落ちてよい」とき
- 例示用の短いサンプルコード
- 固定文字列の `parse()` など、失敗したら実装ミスと言えるとき
- 初期化時に必須データが壊れていたら続行不能なとき

例えばテストでは:

```rust
#[test]
fn parse_number() {
    let n: i32 = "123".parse().unwrap();
    assert_eq!(n, 123);
}
```

のように書くことはよくあります。

## どんなときに避けるべきか

次のような場所での `unwrap()` は雑になりやすいです。

- ユーザー入力
- ファイル読み書き
- ネットワーク
- ライブラリ内部の公開API
- 長く動くサーバーやツール本体

こういう場所では失敗が普通にありえるので、
`match`, `if let`, `?`, `unwrap_or`, `unwrap_or_else` などを先に考える方が自然です。

## 迷ったときの基準

- 「失敗は利用者や環境の都合でも普通に起こる」なら `unwrap()` しない
- 「失敗したらプログラムを書く側のバグっぽい」なら `expect()` を検討する
- 「上に返せる」なら `?` を優先する
- 「値がない場合の代替値がある」なら `unwrap_or(...)` を考える

## 今の段階での理解のゴール

- `unwrap()` は `Option` / `Result` の中身を強引に取り出す道具
- 失敗時は回復せず `panic` する
- `expect()` は同じ動きで、理由をメッセージに残せる
- 回復可能な失敗には `unwrap()` より `match` や `?` が向いている
