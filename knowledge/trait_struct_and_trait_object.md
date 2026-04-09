# `trait` と `struct`、`trait bound` と `trait object`

このノートは、`trait` の基礎を読んだあとに混ざりやすい
2 つの違いを切り分けるためのものです。

## まず一言で

- `struct` は「データの形」
- `trait` は「できることの約束」
- `trait bound` は「この約束を満たす型だけ受け付ける」というコンパイル時の条件
- `trait object` は「具体的な型は隠したまま、その約束だけで扱う」という実行時の入れ物

## `struct` と `trait` の違い

```rust
struct Dog {
    name: String,
}

trait Speak {
    fn speak(&self);
}
```

ここでの役割はまったく違います。

- `Dog` はメモリに何を持つかを表す
- `Speak` は「`speak` が呼べる」という約束を表す

`trait` 自体はフィールドを持ちません。
つまり `trait` は「データ本体」ではなく、
「この型にはこういう操作があります」という説明書です。

C でたとえると:

- `struct Dog` は普通の `struct`
- `trait Speak` は関数ポインタ表を含む interface の約束に近い

ただし C と違って、
Rust では「その約束を満たしているか」を型システムが検査します。

## `impl Dog` と `impl Speak for Dog`

```rust
impl Dog {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{}: wan", self.name);
    }
}
```

違いは次です。

- `impl Dog { ... }` は `Dog` 固有のメソッド
- `impl Speak for Dog { ... }` は `Dog` が `Speak` の約束を満たす定義

## `trait bound` とは何か

```rust
fn make_it_speak<T: Speak>(x: &T) {
    x.speak();
}
```

`T: Speak` が trait bound です。
意味は:

- `T` は何でもよいわけではない
- `Speak` を実装している型だけ受け付ける
- 関数の中では `Speak` が保証する操作だけ使える

これはコンパイル時の条件です。
`Dog` 用、`Cat` 用のように、
具体的な型ごとに別々にコードが作られる形で使われることが多いです。

```rust
struct Cat;

impl Speak for Cat {
    fn speak(&self) {
        println!("nyan");
    }
}

make_it_speak(&Dog { name: String::from("pochi") });
make_it_speak(&Cat);
```

この 2 回の呼び出しでは、
それぞれ具体的な型は最初から分かっています。

## `trait object` とは何か

```rust
fn make_it_speak_dyn(x: &dyn Speak) {
    x.speak();
}
```

`&dyn Speak` が trait object です。
意味は:

- 呼び出し側の具体的な型 `Dog` や `Cat` は隠す
- ただし `Speak` は満たしている、とだけ分かっている
- 実際にどの `speak` を呼ぶかは実行時に決まる

これは C でいうと
「データへのポインタ」と「どの関数を呼ぶかの表」を
一緒に持つ形にかなり近いです。

## なぜ `dyn Speak` は `&` や `Box` の後ろに出るのか

一番大事な理由は、
`dyn Speak` 自体のサイズがコンパイル時に分からないからです。

Rust では、普通のローカル変数や引数は
「何バイト必要か」がコンパイル時に分かっている必要があります。

```rust
let n: i32 = 10;
```

これは `i32` が 4 バイトだと最初から分かるので置けます。

しかし `dyn Speak` は、
実体が `Dog` か `Cat` か別の型か分かりません。
型ごとにサイズが違うので、
`dyn Speak` 単体では必要な大きさを決められません。

そのため、次のように「サイズが分かる入れ物」を使います。

- `&dyn Speak`
- `Box<dyn Speak>`
- `Rc<dyn Speak>`
- `Arc<dyn Speak>`

たとえば `&dyn Speak` なら、
置いているのは `dyn Speak` 本体ではなく
「どこに実体があるかを指す参照」です。
参照そのもののサイズは決まっているので、
ローカル変数に置けます。

## `&dyn Speak` が実際に持っているもの

感覚的には `&dyn Speak` は次の 2 つを持ちます。

- 実データへのポインタ
- その型用のメソッド表へのポインタ

だから実行時に
「今の中身は `Dog` だから `Dog` 用の `speak` を呼ぶ」
ができます。

これは普通の `&T` より少し情報が多い参照です。

## `str` や `[T]` と同じ仲間だと思うと分かりやすい

`dyn Speak` は特別に変わった例ではなく、
Rust にある「サイズ不明な型」の 1 つです。

たとえば:

- `str`
- `[i32]`
- `dyn Speak`

はどれも単体では扱いにくく、
ふつうは次のように使います。

- `&str`
- `&[i32]`
- `&dyn Speak`

つまり
「本体のサイズは分からないので、まず参照や所有ポインタ越しに扱う」
という点で共通しています。

## `Box<dyn Speak>` と `&dyn Speak>` の違い

- `&dyn Speak`
  借りて使うだけ
- `Box<dyn Speak>`
  所有権ごと持つ

```rust
fn borrowed(x: &dyn Speak) {
    x.speak();
}

fn owned(x: Box<dyn Speak>) {
    x.speak();
}
```

前者は「誰かが持っているものを借りる」形で、
後者は「その値の持ち主になる」形です。

## `dyn Trait` をそのまま引数にしない理由

次は書けません。

```rust
fn bad(x: dyn Speak) {
    x.speak();
}
```

`x` をスタック上にどう置くか決められないからです。

代わりに:

```rust
fn ok(x: &dyn Speak) {
    x.speak();
}
```

のように、
サイズが分かる参照やポインタ経由で受け取ります。

## ここで出てくる `Sized` とは何か

Rust では、多くの場所で
「その型のサイズはコンパイル時に分かる」
という `Sized` の前提があります。

たとえばジェネリクス `T` も、
最初はだいたい次の意味で読んで構いません。

```rust
fn show<T: Speak>(x: &T) {
    x.speak();
}
```

これは感覚的には:

```rust
fn show<T: Speak + Sized>(x: &T) {
    x.speak();
}
```

に近いです。

`dyn Speak` は `Sized` ではないので、
「`T` は普通のサイズ既知型だ」と思っている generic に
そのまま入れることはできません。

もしサイズ不明型も受けたいなら、
`?Sized` を付けます。

```rust
fn show<T: Speak + ?Sized>(x: &T) {
    x.speak();
}
```

こうすると `&Dog` も `&dyn Speak` も受けられます。

## 何がうれしいのか

trait object を使うと、
違う具体型を 1 つのコレクションに入れやすくなります。

```rust
let animals: Vec<Box<dyn Speak>> = vec![
    Box::new(Dog { name: String::from("pochi") }),
    Box::new(Cat),
];

for animal in &animals {
    animal.speak();
}
```

`Dog` と `Cat` は別の型なので、
`Vec<Dog>` に `Cat` は入れられません。
そこで「具体型は隠して、`Speak` できるものとして扱う」ために
`Box<dyn Speak>` を使います。

## `trait bound` と `trait object` の違い

同じ「`Speak` できるもの」を扱いますが、役割は違います。

- `T: Speak`
  呼び出す時点で具体型が決まっている
- `dyn Speak`
  具体型は隠して、共通の interface だけで扱う

- `T: Speak`
  コンパイル時に型ごとに最適化しやすい
- `dyn Speak`
  実行時にどの実装を呼ぶか決める

- `T: Speak`
  1 回の呼び出しで `T` は 1 つの具体型
- `dyn Speak`
  `Dog` と `Cat` のような異なる型を同じ入れ物で扱いやすい

## まずはこう覚える

- `struct` はデータの形
- `trait` は操作の約束
- `trait bound` はジェネリクスに付ける条件
- `trait object` は具体型を隠して共通操作だけで扱う仕組み

最初は、

- 具体型が最初から分かっているなら `T: Trait`
- いろいろな型をまとめて持ちたいなら `dyn Trait`

という判断で十分です。

さらに、

- `dyn Trait` はサイズ不明なので単体では置きにくい
- だから `&dyn Trait` や `Box<dyn Trait>` の形で使う

まで言えれば十分です。
