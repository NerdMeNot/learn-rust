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

    fn balance(&self) -> u32 {
        self.balance
    }

    fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }

    fn close(self) -> String {
        format!("{} closed with {}", self.owner, self.balance)
    }
}

fn main() {
    let mut acct = Account::new("ana");
    acct.deposit(50);
    acct.deposit(25);
    println!("balance: {}", acct.balance());
    let note = acct.close();
    println!("note: {note}");
}
