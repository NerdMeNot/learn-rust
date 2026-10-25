mod bank {
    pub struct Account {
        balance: u32,
    }

    impl Account {
        pub fn new() -> Self {
            Account { balance: 0 }
        }

        pub fn deposit(&mut self, amount: u32) {
            self.balance += amount;
        }

        pub fn balance(&self) -> u32 {
            self.balance
        }
    }
}

fn main() {
    let mut acct = bank::Account::new();
    acct.deposit(5);
    acct.balance = 1_000_000;
    println!("balance: {}", acct.balance());
}
