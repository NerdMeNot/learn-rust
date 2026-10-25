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

fn top_up(acct: &mut Account) {
    acct.deposit(10); // `acct` is already a `&mut Account`; no `(*acct)` needed
}

fn main() {
    let mut acct = Account { balance: 0 };
    acct.deposit(5); // the compiler passes `&mut acct`
    top_up(&mut acct);
    println!("value:       {}", acct.balance());

    let r = &acct;
    let rr = &r;
    println!("&Account:    {}", r.balance());
    println!("&&Account:   {}", rr.balance());

    let mut accounts = vec![Account { balance: 1 }, Account { balance: 2 }];
    for a in &mut accounts {
        a.deposit(100); // `a` is a `&mut Account`
    }
    println!("accounts[0]: {}", accounts[0].balance());
    println!("accounts[1]: {}", accounts[1].balance());
}
