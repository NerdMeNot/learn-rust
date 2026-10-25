struct Account {
    balance: u32,
}

impl Account {
    fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }
}

fn main() {
    let mut acct = Account { balance: 0 };
    let f = acct.deposit;
    f(5);
}
