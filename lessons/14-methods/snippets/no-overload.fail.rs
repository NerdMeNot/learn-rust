struct Account {
    owner: String,
    balance: u32,
}

impl Account {
    fn new(owner: &str) -> Self {
        Self {
            owner: owner.to_string(),
            balance: 0,
        }
    }

    fn new(owner: &str, balance: u32) -> Self {
        Self {
            owner: owner.to_string(),
            balance,
        }
    }
}

fn main() {
    let a = Account::new("ana");
    println!("{} has {}", a.owner, a.balance);
}
