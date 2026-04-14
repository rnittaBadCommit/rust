// 口座の入出金を表す型を書く課題です。
//
// 自分で次の `struct` と `impl` を書いてください。
//
// - `struct BankAccount { owner: String, balance: i64 }`
// - `deposit(&mut self, amount: i64) -> Result<(), &'static str>`
// - `withdraw(&mut self, amount: i64) -> Result<(), &'static str>`
// - `balance(&self) -> i64`
//
// 条件:
// - `deposit` は残高を増やす
// - `withdraw` は残高不足なら `Err(...)`
// - 失敗時は残高を変えない
//
// テストは通常のコードとして置いてあります。
// 先に `struct` と `impl` を書いてから `cargo test` してください。

#[cfg(test)]
mod tests {
    use super::BankAccount;

    #[test]
    fn deposit_increases_balance() {
        let mut account = BankAccount {
            owner: String::from("Alice"),
            balance: 100,
        };
        account.deposit(25).unwrap();
        assert_eq!(account.balance(), 125);
    }

    #[test]
    fn withdraw_decreases_balance() {
        let mut account = BankAccount {
            owner: String::from("Bob"),
            balance: 100,
        };
        account.withdraw(40).unwrap();
        assert_eq!(account.balance(), 60);
    }

    #[test]
    fn withdraw_returns_error_when_balance_is_not_enough() {
        let mut account = BankAccount {
            owner: String::from("Carol"),
            balance: 100,
        };
        assert!(account.withdraw(120).is_err());
        assert_eq!(account.balance(), 100);
    }
}
