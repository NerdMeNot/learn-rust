mod bank {
    pub struct Account {
        balance: i64,
    }

    pub struct Auditor;

    impl Account {
        pub fn new(balance: i64) -> Account {
            Account { balance }
        }
    }

    impl Auditor {
        // A different type, but the same module: it can read the private field.
        pub fn peek(&self, a: &Account) -> i64 {
            a.balance
        }
    }

    pub fn open() -> (Account, Auditor) {
        (Account::new(250), Auditor)
    }
}

fn main() {
    let (account, auditor) = bank::open();
    println!("peek: {}", auditor.peek(&account));
}
