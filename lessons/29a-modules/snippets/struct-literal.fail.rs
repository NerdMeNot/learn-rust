mod account {
    pub struct Account {
        pub currency: &'static str,
        balance_cents: i64,
    }

    impl Account {
        pub fn balance(&self) -> i64 {
            self.balance_cents
        }
    }
}

fn main() {
    // Skips any check a constructor would make.
    let acct = account::Account {
        currency: "EUR",
        balance_cents: -999,
    };
    println!("{} {}", acct.currency, acct.balance());
}
