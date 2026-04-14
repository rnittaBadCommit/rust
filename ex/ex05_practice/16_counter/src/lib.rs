// カウンタ型を作る課題です。
//
// 自分で次の `struct` と `impl` を書いてください。
//
// - `struct Counter`
// - `Counter::new() -> Self`
// - `inc(&mut self)`
// - `get(&self) -> i32`
// - `reset(&mut self)`
//
// 条件:
// - 内部状態は `i32` でよい
// - `inc` で 1 増やす
// - `reset` で 0 に戻す
//
// テストは通常のコードとして置いてあります。
// 先に `struct` と `impl` を書いてから `cargo test` してください。

#[cfg(test)]
mod tests {
    use super::Counter;

    #[test]
    fn new_counter_starts_at_zero() {
        let counter = Counter::new();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn inc_increases_value() {
        let mut counter = Counter::new();
        counter.inc();
        counter.inc();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn reset_brings_value_back_to_zero() {
        let mut counter = Counter::new();
        counter.inc();
        counter.inc();
        counter.reset();
        assert_eq!(counter.get(), 0);
    }
}
