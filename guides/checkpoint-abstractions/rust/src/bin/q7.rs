mod bank {
    pub struct Account {
        balance: i64,
    }

    impl Account {
        pub fn new() -> Account {
            Account { balance: 0 }
        }

        pub fn balance(&self) -> i64 {
            self.balance
        }
    }
}

fn main() {
    let rich = bank::Account { balance: 1_000_000 };
    let fresh = bank::Account::new();
    println!("{}", rich.balance());
    println!("{}", fresh.balance());
}
