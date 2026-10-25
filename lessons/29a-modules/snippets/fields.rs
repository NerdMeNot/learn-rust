mod account {
    pub struct Account {
        pub currency: &'static str,
        balance_cents: i64, // private: only code in `account` can touch it
    }

    impl Account {
        // The only way to build an Account from outside, so the rule below holds.
        pub fn new(cents: i64) -> Option<Account> {
            if cents < 0 {
                return None;
            }
            Some(Account {
                currency: "EUR",
                balance_cents: cents,
            })
        }

        pub fn balance(&self) -> i64 {
            self.balance_cents
        }

        pub fn withdraw(&mut self, cents: i64) -> bool {
            if cents > self.balance_cents {
                return false;
            }
            self.balance_cents -= cents;
            true
        }
    }
}

fn main() {
    println!("negative start: {}", account::Account::new(-5).is_none());
    let mut acct = account::Account::new(500).unwrap();
    println!("currency: {}", acct.currency);
    println!("withdraw 200: {}", acct.withdraw(200));
    println!("withdraw 900: {}", acct.withdraw(900));
    println!("balance: {}", acct.balance());
}
