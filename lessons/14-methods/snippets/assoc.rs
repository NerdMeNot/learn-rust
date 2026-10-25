struct Account {
    owner: String,
    balance: u32,
}

impl Account {
    const MAX_BALANCE: u32 = 1_000;

    fn new(owner: &str) -> Self {
        Self::with_balance(owner, 0)
    }

    fn with_balance(owner: &str, balance: u32) -> Self {
        Self {
            owner: owner.to_string(),
            balance: balance.min(Self::MAX_BALANCE),
        }
    }
}

// A second `impl` block for the same type is allowed.
impl Account {
    fn is_full(&self) -> bool {
        self.balance == Self::MAX_BALANCE
    }
}

fn main() {
    let a = Account::new("ana");
    let b = Account::with_balance("bo", 5_000);
    println!("a: {} has {}, full: {}", a.owner, a.balance, a.is_full());
    println!("b: {} has {}, full: {}", b.owner, b.balance, b.is_full());
    println!("limit: {}", Account::MAX_BALANCE);
}
