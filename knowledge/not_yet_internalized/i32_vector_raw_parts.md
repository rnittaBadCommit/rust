# `Vec<i32>` を自作するときの基本形

`Vec<T>` の最小構造は、C の動的配列とかなり近いです。

```text
struct I32Vector {
    ptr: *mut i32,
    len: usize,
    cap: usize,
}
```

- `ptr`: heap 上に確保した領域の先頭
- `len`: すでに初期化済みで、読んでよい要素数
- `cap`: 確保済みの要素数

重要なのは、`len` と `cap` が違うことです。

```text
0            len              cap
| initialized | uninitialized |
```

`0..len` は `i32` として有効な値が入っています。
`len..cap` は領域だけ確保されていますが、まだ `i32` として読んではいけません。

## `push`

`push` は次の順で動きます。

1. `len == cap` なら、より大きい領域を確保する
2. 古い要素を新しい領域へコピーする
3. 古い領域を解放する
4. `ptr.add(len)` に新しい値を書き込む
5. `len += 1`

Rust では未初期化領域へ代入するとき、`*ptr = value` より `ptr.write(value)` が自然です。
「そこにはまだ有効な値がない」と分かる書き方だからです。

## `pop`

`pop` は末尾から値を取り出します。

1. `len == 0` なら `None`
2. `len -= 1`
3. `ptr.add(len).read()` で値を読み出す
4. `Some(value)` を返す

`read()` は所有権をメモリから取り出す操作です。
今回の `i32` は `Copy` なのでかなり単純ですが、将来 `String` などに広げるときは drop との関係が重要になります。

## `Drop`

`I32Vector` 自体が破棄されるときは、確保した heap 領域を `dealloc` します。

今回の `i32` 限定版では、各要素に対する drop は不要です。
ただし generic な `Vector<T>` に進むなら、`0..len` の初期化済み要素だけを `drop_in_place` してから領域を解放する必要があります。

## zero capacity

`cap == 0` のときは heap 確保をしません。
ただし slice を作る都合などで、ポインタは null ではなく `NonNull::dangling()` のような「非 null かつアライン済みだが、読んではいけない番地」を持たせます。

これは C の `NULL` とは違う発想です。
Rust の slice は長さ 0 でも、ポインタに非 null と alignment を要求するためです。

### C ならどう書きがちか

C の動的配列なら、空の状態をこう表すことが多いです。

```c
typedef struct {
    int *ptr;
    size_t len;
    size_t cap;
} I32Vector;

I32Vector v = {
    .ptr = NULL,
    .len = 0,
    .cap = 0,
};
```

この形でも、`len == 0` の間に `ptr[0]` を読まなければ問題ありません。
`push` するときは、先に `malloc` / `realloc` してから書き込みます。

Rust でも「`cap == 0` のときは heap を確保しない」という方針は同じです。
違うのは、内部ポインタとして `NULL` を使わないことです。

### なぜ null ではだめなのか

今回の実装では、空 vector でも `as_slice()` を呼べます。

```rust
let v = I32Vector::new();
assert_eq!(v.as_slice(), &[]);
```

`as_slice()` の中では、概念的に次のようなことをしています。

```rust
slice::from_raw_parts(ptr, len)
```

ここで `len == 0` なら実際には何も読みません。
しかし Rust の slice `&[i32]` は、長さ 0 でも内部の data pointer が

1. null ではない
2. `i32` の alignment を満たしている

という条件を要求します。

つまり、C の感覚で

```text
ptr = NULL
len = 0
```

から空 slice を作る、という形は Rust では不適切です。

### `NonNull::dangling()` とは何か

`NonNull::dangling()` は「今は本物の heap 領域を指していないが、null ではなく、型 `T` の alignment は満たす」ダミーのポインタを作ります。

名前に `dangling` とある通り、その番地を読んだり書いたりしてはいけません。
使ってよいのは、`len == 0` なので実際にはアクセスしない場面だけです。

今回の `I32Vector::new()` はこういう状態です。

```text
ptr = dangling, len = 0, cap = 0
```

この状態で許されること:

```text
len を見る
cap を見る
空 slice を作る
push の前に grow する
drop で何も解放しない
```

この状態でしてはいけないこと:

```text
ptr を読む
ptr に書く
ptr を dealloc する
```

### `push` ではどう安全にしているか

空 vector に `push` するとき、最初の状態はこうです。

```text
ptr = dangling
len = 0
cap = 0
```

`push` はまず `len == cap` を見ます。
空なら `0 == 0` なので、先に `grow()` します。

```text
grow 前:
ptr = dangling, len = 0, cap = 0

grow 後:
ptr = heap allocation, len = 0, cap = 4
```

その後で初めて `ptr.add(len).write(value)` します。
つまり、`dangling` のまま書き込む経路はありません。

### `Drop` ではどう安全にしているか

`cap == 0` のときは heap 確保をしていません。
そのため、`drop` では何も解放してはいけません。

```rust
if self.cap == 0 {
    return;
}
```

この判定があるので、`NonNull::dangling()` を `dealloc` に渡しません。

`dealloc` してよいのは、過去に `alloc` で確保した本物の pointer だけです。

### まとめ

`cap == 0` のとき:

```text
heap 領域はない
だから dealloc してはいけない
しかし Rust の slice 用には null ではない pointer が必要
なので NonNull::dangling() を placeholder として持つ
```

`NonNull::dangling()` は「空 vector 用の安全な番地」ではありません。
正確には「アクセスしないことを前提に、Rust のポインタ不変条件だけを満たすダミー」です。

## `NonNull<T>` は「null ではない raw pointer」

今回の実装では、内部ポインタを `*mut i32` ではなく `NonNull<i32>` で持っています。

```rust
pub struct I32Vector {
    ptr: NonNull<i32>,
    len: usize,
    cap: usize,
}
```

`NonNull<T>` は、名前の通り「null ではない」ことを型で表す raw pointer です。
ただし、これだけで安全に読めるわけではありません。

- null ではない
- `T` の alignment を満たすことを期待する
- しかし、指している先が初期化済みかどうかは保証しない
- 参照 `&T` / `&mut T` と違って、借用規則を自動では守らない

つまり `NonNull<i32>` は、
`Option<NonNull<i32>>` にせず「必ず何らかの非 null pointer を持つ」
という内部表現にしたいときに使います。

実際に pointer 演算や読み書きをするときは、
`self.ptr.as_ptr()` で `*mut i32` に戻しています。

## `with_capacity` は「確保するが初期化しない」

`with_capacity(capacity)` は、要素数 `len` を増やさずに、
先に `cap` だけ確保します。

```rust
let mut v = I32Vector::with_capacity(2);

assert_eq!(v.len(), 0);
assert_eq!(v.capacity(), 2);
```

これは C でいうと、
`malloc(sizeof(int) * capacity)` はしたが、
まだ `int` の値として読んでよい要素は 0 個、
という状態です。

```text
0                              cap
| all allocated, uninitialized |
len = 0
```

したがって、`capacity()` は確保済み領域の大きさ、
`len()` は初期化済みで読んでよい要素数です。

`capacity == 0` のときは heap 確保をせず、
`new()` と同じ dangling pointer の状態に戻します。

## `Layout` は allocator に渡す「サイズと alignment」

Rust の低レベル allocator API は、
`malloc(size)` のようにサイズだけを渡すのではなく、
`Layout` を渡します。

```rust
let layout = Layout::array::<i32>(capacity).expect("capacity overflow");
let raw = unsafe { alloc(layout) as *mut i32 };
```

`Layout` は主に次を持つ値です。

- 何バイト必要か
- 何バイト alignment が必要か

alignment は、
「その型の値を置くアドレスが何バイト境界にそろっている必要があるか」
という条件です。

例えば `i32` は多くの環境で 4 byte alignment です。
これは、`i32` の値を置くアドレスが 4 の倍数である必要がある、
という意味です。

```text
OK:  address 0x1000  // 4 で割り切れる
NG:  address 0x1001  // 4 で割り切れない
```

C でいうと、
`int32_t *p` として読むなら、
`p` は `int32_t` として正しくそろったアドレスを指していなければならない、
という前提に近いです。

alignment が合っていない pointer から値を読むことは、
Rust では未定義動作になります。
CPU によっては遅いだけで動くこともありますが、
Rust の unsafe API では「動く環境もある」ではなく
「型の前提を満たしているか」を基準に考えます。

`Layout::array::<i32>(capacity)` は、
`i32` を `capacity` 個並べるための layout を作ります。

つまりこの layout には、
「`i32` を `capacity` 個ぶん置けるサイズ」と
「`i32` として読める alignment」
の両方が入っています。

C で雑に書くなら次に近いです。

```c
malloc(sizeof(int32_t) * capacity);
```

ただし Rust では、
`sizeof(i32) * capacity` が `usize` を overflow する場合も
`Layout::array` が検出してくれます。

`dealloc` するときは、
確保時と同じ layout を渡す必要があります。

```rust
let layout = array_layout(self.cap);
unsafe {
    dealloc(self.ptr.as_ptr() as *mut u8, layout);
}
```

これは C の `free(ptr)` より情報が多い API です。
Rust の allocator は「この pointer はこの layout で確保された」
という前提で解放します。

## `alloc` の失敗は null pointer で返る

`std::alloc::alloc(layout)` は、
成功すると確保した領域への pointer を返します。
失敗すると null pointer を返すことがあります。

そのため実装では次のようにしています。

```rust
let raw = unsafe { alloc(layout) as *mut i32 };
let ptr = NonNull::new(raw).unwrap_or_else(|| handle_alloc_error(layout));
```

読み方は次です。

1. `alloc(layout)` で `*mut u8` を得る
2. `as *mut i32` で `i32` 用 pointer として扱う
3. `NonNull::new(raw)` で null なら `None`、非 null なら `Some(NonNull<i32>)`
4. null なら `handle_alloc_error(layout)` に渡して、確保失敗として止める

`handle_alloc_error` は、普通の入力エラーを `Result` で返すような処理ではありません。
「メモリ確保に失敗したので、このまま続行できない」という扱いです。

## `grow` では overflow とコピー範囲を分けて考える

`grow` は容量を増やします。

```rust
let new_cap = if self.cap == 0 {
    4
} else {
    self.cap.checked_mul(2).expect("capacity overflow")
};
```

`checked_mul(2)` は、
`self.cap * 2` が `usize` を overflow したら `None` を返します。
そこで `expect("capacity overflow")` して、
容量として表せない場合は panic させています。

新しい領域を確保したら、
古い領域のうち `0..len` の初期化済み要素だけをコピーします。

```rust
ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr.as_ptr(), self.len);
```

`copy_nonoverlapping` は C の `memcpy` に近いです。

- コピー元とコピー先が重なってはいけない
- 第 3 引数はバイト数ではなく、要素数
- 今回なら `self.len` 個の `i32` をコピーする

ここで `cap` 個ではなく `len` 個だけコピーするのが重要です。
`len..cap` は未初期化なので、`i32` として読んではいけません。

## `as_slice` / `as_mut_slice` は unsafe を安全な API に閉じ込める

`as_slice` は raw pointer と長さから `&[i32]` を作ります。

```rust
pub fn as_slice(&self) -> &[i32] {
    unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
}
```

`slice::from_raw_parts` 自体は unsafe です。
理由は、呼び出し側が次を守らなければならないからです。

- pointer が null ではない
- pointer が `i32` の alignment を満たす
- `len` 個ぶんのメモリが有効
- `0..len` が初期化済み
- 返す slice の寿命中、その参照としてのルールを破らない

`I32Vector` は内部でこの条件を守るようにしているので、
外側には安全な `as_slice(&self) -> &[i32]` として公開できます。

`as_mut_slice` も同じですが、こちらは `&mut self` を取ります。

```rust
pub fn as_mut_slice(&mut self) -> &mut [i32]
```

`&mut self` にすることで、
呼び出し中に同じ vector へ別の共有・可変参照を同時に作りにくくしています。
raw pointer の危険な部分を、Rust の普通の借用 API の内側へ閉じ込めている形です。

## `get` / `get_mut` は `cap` ではなく `len` で境界チェックする

`get` は範囲外なら `None` を返します。

```rust
pub fn get(&self, index: usize) -> Option<&i32> {
    if index >= self.len {
        return None;
    }

    Some(unsafe { &*self.ptr.as_ptr().add(index) })
}
```

ここでチェックするのは `index >= self.cap` ではありません。
読んでよいのは初期化済みの `0..len` だけなので、
`index >= self.len` を範囲外とします。

`get_mut` も同じ考え方です。

```rust
pub fn get_mut(&mut self, index: usize) -> Option<&mut i32>
```

返すものが `&mut i32` なので、メソッド自体も `&mut self` を取ります。
これは C の `int *` を返す感覚に近いですが、
Rust では「可変参照を返すなら、元の container も可変借用する」
という形で aliasing を制限します。

## `Index` / `IndexMut` は `[]` 演算子につながる trait

Rust の `v[0]` は、`Index` trait に結びついています。

```rust
impl Index<usize> for I32Vector {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}
```

読み方は次です。

- `Index<usize>`: `usize` で添字アクセスできる
- `type Output = i32`: `v[index]` で見える要素型は `i32`
- `index(...) -> &Self::Output`: 添字アクセスは要素への共有参照を返す

範囲外の場合、
`get` は `None` を返しますが、
`[]` は panic するのが標準的な挙動です。
そのため `expect("index out of bounds")` で panic に変換しています。

書き換え可能な `v[1] = 80` には `IndexMut` が必要です。

```rust
impl IndexMut<usize> for I32Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}
```

`IndexMut` は `Index` の上に乗る trait なので、
`type Output = i32` は `Index` 側で定義したものを使います。

## `Default` は「引数なしの自然な初期値」

`Default` は、その型の自然な初期値を作る trait です。

```rust
impl Default for I32Vector {
    fn default() -> Self {
        Self::new()
    }
}
```

この実装により、次の 2 つは同じ意味になります。

```rust
let a = I32Vector::new();
let b = I32Vector::default();
```

`Default` があると、
generic なコードや `unwrap_or_default()` のような API からも
その型を初期化しやすくなります。

## `Debug` を手で実装すると、見せ方を選べる

`#[derive(Debug)]` ではなく手で `fmt::Debug` を実装すると、
どのフィールドをどう表示するかを選べます。

```rust
impl fmt::Debug for I32Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("I32Vector")
            .field("items", &self.as_slice())
            .field("len", &self.len)
            .field("capacity", &self.cap)
            .finish()
    }
}
```

ここでは raw pointer そのものではなく、
利用者に意味がある `items`, `len`, `capacity` を表示しています。

C でいうと、
内部構造をそのまま dump するのではなく、
デバッグ用の表示関数を自分で書いている感覚です。

`fmt::Formatter<'_>` の `'_` は、
「ここでは具体的なライフタイム名を付けず、コンパイラに推論させる」
という省略記法です。

## `#[cfg(test)]` と `#[should_panic]`

`#[cfg(test)]` は、
`cargo test` のときだけその module をコンパイルするための属性です。

```rust
#[cfg(test)]
mod tests {
    // tests
}
```

通常の library 利用時には、
この `tests` module はコンパイル対象から外れます。

`#[should_panic(expected = "...")]` は、
そのテストが panic することを期待する属性です。

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn index_panics_when_out_of_bounds() {
    let v = I32Vector::new();
    let _ = v[0];
}
```

これは「panic しないこと」ではなく、
「範囲外添字で panic すること」が仕様である場合に使います。

## `unsafe` は実装内部の約束を人間が守る場所

この `I32Vector` は外側から見ると安全な API ですが、
内部では複数の `unsafe` を使っています。

主な約束は次です。

- `ptr.add(index)` するとき、`index` が確保済み範囲内にある
- `read()` する場所は初期化済みである
- `write()` する場所は書き込んでよい領域である
- `from_raw_parts` に渡す pointer と長さが slice の条件を満たす
- `dealloc` には、確保時と同じ layout を渡す
- 同じ要素に対する `&mut` と他の参照を同時に作らない

Rust では、
`unsafe` ブロックの中だけコンパイラの保証が弱くなります。
しかし、公開 API 全体を `unsafe fn` にしなくてよいように、
実装者が内部不変条件を守って安全なメソッドとして包みます。

この形を「unsafe を安全な抽象化の内側へ閉じ込める」と考えると分かりやすいです。
