# `Option::filter()` と `Iterator::filter()`

このノートは、同じ `filter` という名前でも
`Iterator` と `Option` で何が違うのかを整理するためのものです。

## まず一言で

- `Iterator::filter()` は、各要素に条件をかける
- `Iterator::filter()` は、`true` の要素だけを残した新しいイテレータを作る
- `Option::filter()` は、`Some(value)` の中身 1 個に条件をかける
- `Option::filter()` は、条件が `true` なら `Some(value)` のまま返す
- `Option::filter()` は、条件が `false` なら `None` を返す
- `Option::filter()` は、もともと `None` ならそのまま `None` を返す

## 共通点

どちらも感覚としては
「条件をかけて、通るものだけ残す」です。

ただし、対象が違います。

- `Iterator::filter()` は「0 個以上ある列」
- `Option::filter()` は「0 個か 1 個」

です。

## `Iterator::filter()` は何をするか

例えば:

```rust
let xs = [1, 2, 3, 4, 5];

let evens: Vec<i32> = xs
    .into_iter()
    .filter(|x| x % 2 == 0)
    .collect();

assert_eq!(evens, vec![2, 4]);
```

ここでは各要素 `1, 2, 3, 4, 5` に対して
`x % 2 == 0` を調べ、
`true` のものだけを残しています。

ポイントは、
`filter()` 自体はすぐに `Vec` を作るわけではなく、
「条件つきで要素を流す新しいイテレータ」を作ることです。

## `Option::filter()` は何をするか

例えば:

```rust
let x = Some(10);

assert_eq!(x.filter(|n| *n > 5), Some(10));
assert_eq!(x.filter(|n| *n > 20), None);

let y: Option<i32> = None;
assert_eq!(y.filter(|n| *n > 5), None);
```

ここでは `Some(10)` の中にある 1 個の値 `10` に条件をかけています。

- 条件が通るなら `Some(10)` のまま
- 条件が通らないなら `None`
- そもそも `None` ならそのまま `None`

です。

## `Option::filter()` を `match` で書くと

感覚としては次に近いです。

```rust
match opt {
    Some(v) if predicate(&v) => Some(v),
    _ => None,
}
```

つまり
「`Some(v)` だったら追加条件をかける」
という書き方です。

## `split_once(...).filter(...)` はどう読むか

例えば:

```rust
s.split_once(':')
    .filter(|(left, right)| !left.is_empty() && !right.is_empty())
```

は次の順で読みます。

1. `split_once(':')` で `Option<(&str, &str)>` を作る
2. `Some((left, right))` のときだけ条件をかける
3. 条件を満たせば `Some((left, right))` を残す
4. 条件を満たさなければ `None` にする

具体例:

```rust
assert_eq!("abc".split_once(':'), None);
assert_eq!(":8080".split_once(':'), Some(("", "8080")));
assert_eq!("host:8080".split_once(':'), Some(("host", "8080")));
```

このあと `filter(...)` を通すと:

- `"abc"` は最初から `None`
- `":8080"` は `Some(("", "8080"))` だが条件で落ちて `None`
- `"host:8080"` は条件を通って `Some(("host", "8080"))`

になります。

## C と比べると

`Iterator::filter()` は、
C で配列を走査しながら
条件に合うものだけを別の出力へ送る処理に近いです。

```c
for (size_t i = 0; i < n; i++) {
    if (pred(xs[i])) {
        push_out(xs[i]);
    }
}
```

一方 `Option::filter()` は、
C でいうと
「値があるかをまず確認し、
あれば追加条件をかけて、
ダメなら無効扱いにする」
という流れに近いです。

```c
if (p != NULL && pred(*p)) {
    /* keep */
} else {
    /* treat as absent */
}
```

## まずはこう覚える

- `Iterator::filter()` は「たくさんある要素をふるいにかける」
- `Option::filter()` は「あるかないか分からない 1 個を条件つきで残す」
- どちらも `true` のものだけ残すが、相手にしている個数が違う

## 今の段階での理解のゴール

- `Iterator::filter()` と `Option::filter()` は同じ名前でも対象が違う
- `Iterator::filter()` は複数要素向け、`Option::filter()` は 0 個か 1 個向け
- `Option::filter()` は `Some(v)` に追加条件をかける道具として読むと分かりやすい
