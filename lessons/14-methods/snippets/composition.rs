struct Account {
    balance: u32,
}

impl Account {
    fn balance(&self) -> u32 {
        self.balance
    }

    fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }
}

// There is no `extends`. A bigger type holds the smaller one as a field.
struct Savings {
    account: Account,
    rate_percent: u32,
}

impl Savings {
    fn deposit(&mut self, amount: u32) {
        self.account.deposit(amount); // forwarded by hand
    }

    fn interest(&self) -> u32 {
        self.account.balance() * self.rate_percent / 100
    }
}

fn main() {
    let mut s = Savings {
        account: Account { balance: 0 },
        rate_percent: 5,
    };
    s.deposit(200);
    println!("balance:  {}", s.account.balance());
    println!("interest: {}", s.interest());
}
