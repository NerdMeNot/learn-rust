mod account {
    pub struct Account {
        pub currency: &'static str,
        balance_cents: i64, // no `pub`
    }

    impl Account {
        pub fn new(cents: i64) -> Account {
            Account {
                currency: "EUR",
                balance_cents: cents,
            }
        }

        pub fn balance(&self) -> i64 {
            self.balance_cents
        }
    }
}

fn main() {
    let acct = account::Account::new(500);
    println!("currency: {}", acct.currency);
    println!("balance: {}", acct.balance_cents);
}
