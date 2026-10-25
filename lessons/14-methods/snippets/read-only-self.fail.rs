struct Account {
    balance: u32,
}

impl Account {
    fn reset(&self) {
        self.balance = 0;
    }
}

fn main() {
    let acct = Account { balance: 5 };
    acct.reset();
    println!("balance: {}", acct.balance);
}
