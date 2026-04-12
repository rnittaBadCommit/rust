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

## メモリの見え方を図で書くと

`&Dog` は、感覚的には 1 本のポインタです。

```text
stack
+---------+
| ptr ----|----> Dog { name: ... }
+---------+
```

一方 `&dyn Speak` は、感覚的には 2 ワードの参照です。

```text
stack
+----------+    +------------------+
| data_ptr |--->| Dog { name: ... }|
+----------+    +------------------+
| vtbl_ptr |--->| speak = Dog::... |
+----------+    | drop  = Dog::... |
                | size  = ...      |
                | align = ...      |
                +------------------+
```

ここで大事なのは:

- 実データ本体は `Dog`
- vtable は型ごとの共有表
- `&dyn Speak` 自体は「データ本体」ではなく 2 つのポインタ

という点です。

## `Box<dyn Speak>` はどこに何があるか

`Box<dyn Speak>` も感覚的には
「データへのポインタ」と「vtable へのポインタ」を持ちます。

```text
stack
+----------+    heap
| data_ptr |--->+------------------+
+----------+    | Dog { name: ... }|
| vtbl_ptr |--->+------------------+
+----------+
```

つまり heap に置かれるのは具体型 `Dog` の値そのものです。
vtable まで heap に複製されるわけではありません。
vtable は別の共有領域にある定数表だと思えば十分です。

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

参照に付く追加情報も少し違います。

- `&str`
  データポインタ + 長さ
- `&[i32]`
  データポインタ + 長さ
- `&dyn Speak`
  データポインタ + vtable ポインタ

この「ポインタ + 追加情報」を
fat pointer, wide pointer と呼ぶことがあります。

## `&str` / `&[T]` / `&dyn Trait` の metadata は何が違うか

3 つとも
「data pointer + metadata」
という点では同じです。
違うのは metadata の意味です。

```text
&str
  = data_ptr + len

&[T]
  = data_ptr + len

&dyn Speak
  = data_ptr + vtable_ptr
```

### `&str`

- `data_ptr`
  UTF-8 バイト列の先頭
- `len`
  何バイト分あるか

```text
stack
+----------+    +----------------------+
| data_ptr |--->| h  e  l  l  o        |
+----------+    +----------------------+
| len = 5  |
+----------+
```

ここで必要なのは
「どこから始まるか」と「何バイト読むか」です。

### `&[T]`

- `data_ptr`
  先頭要素へのポインタ
- `len`
  要素数

```text
stack
+----------+    +----------------------+
| data_ptr |--->| 10 | 20 | 30 | 40    |
+----------+    +----------------------+
| len = 4  |
+----------+
```

`&str` と似ていますが、
こちらの長さは「バイト数」ではなく「要素数」です。

### `&dyn Trait`

- `data_ptr`
  具体型の値へのポインタ
- `vtable_ptr`
  その具体型を trait として扱うための表

```text
stack
+----------+    +------------------+
| data_ptr |--->| Dog { ... }      |
+----------+    +------------------+
| vtbl_ptr |--->| speak = Dog::... |
+----------+    | drop  = Dog::... |
                | size  = ...      |
                | align = ...      |
                +------------------+
```

ここでは長さではなく、
「この値を trait としてどう操作するか」が metadata です。

## なぜ slice は長さで、trait object は vtable なのか

必要な追加情報が違うからです。

- `str` や `[T]`
  末尾がどこか分からないので長さが必要
- `dyn Trait`
  メソッド実装や drop の仕方が型ごとに違うので vtable が必要

つまり:

- slice 系は「どこまで読むか」の metadata
- trait object は「どう振る舞うか」の metadata

です。

## C と比べると

かなり雑に対応を取ると:

- `&[T]`
  `T *ptr` + `size_t len`
- `&str`
  `char *ptr` + `size_t len`
- `&dyn Trait`
  `void *data` + 関数表ポインタ

に近いです。

ただし Rust では、
これらをただの慣習ではなく
型システムが 1 つの参照型として管理しているのが重要です。

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

## `?Sized` にしたら何が変わるのか

`?Sized` は
「`T` はサイズ既知でなくてもよい」
と compiler に伝えるだけです。

つまり:

- `dyn Speak` が急にサイズ既知になるわけではない
- compiler が `T = dyn Speak` という具体化も許すようになる

ここで変わるのは、generic の前提です。

`T: Speak` だけだと、compiler は
「`T` は `Sized` でもある」と仮定します。
だから `T = Dog` はよいですが、
`T = dyn Speak` は拒否します。

`T: Speak + ?Sized` にすると、
compiler は `Sized` を仮定しません。
そのかわり、`T` をサイズが必要な場所では雑に扱えなくなります。

## `?Sized` にしても何でもできるわけではない

たとえば次はだめです。

```rust
fn bad<T: Speak + ?Sized>(x: T) {
    x.speak();
}
```

`x: T` は値そのものを受け取るので、
やはりサイズが必要だからです。

つまり `?Sized` を付けた generic では、
`T` は普通こういう形で使います。

- `&T`
- `&mut T`
- `Box<T>`
- `Rc<T>`
- `Arc<T>`
- `*const T`
- `*mut T`

要するに
「`T` 本体を直接置く」のではなく
「`T` へのポインタ越しに扱う」必要があります。

## `?Sized` の内部的な変化を一言でいうと

内部的に一番大きい違いは、
compiler が `T` 用のコードを作るときに
「`size_of::<T>()` が常に分かる」とは仮定しなくなることです。

なので `fn show<T: Speak + ?Sized>(x: &T)` では、
compiler が本当に必要なのは `T` 本体のサイズではなく
`&T` の扱い方だけです。

`T = Dog` のとき:

- `&T` は普通の thin pointer
- `x.speak()` は `Dog` への呼び出し

`T = dyn Speak` のとき:

- `&T` は data ptr + vtable ptr を持つ wide pointer
- `x.speak()` は vtable 経由の動的ディスパッチ

つまり `?Sized` は
「サイズ不明型そのものを直接置けるようにする魔法」
ではなく、
「サイズ不明型をポインタ越しに受ける generic を書けるようにする条件緩和」
です。

## 変わるのは `T` ではなく `&T` の表現

ここは誤解しやすい点です。

`?Sized` を付けても、
`Dog` が別の型になるわけではありません。
`dyn Speak` の実体レイアウトが変わるわけでもありません。

変わるのは、
generic 関数が `T` を受け入れる条件と、
そのとき使われる参照やポインタの表現です。

- `&Dog`
  thin pointer
- `&dyn Speak`
  wide pointer

という違いを compiler が許容できるようになります。

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
