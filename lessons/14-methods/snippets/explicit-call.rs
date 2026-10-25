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

fn main() {
    let mut acct = Account { balance: 0 };
    acct.deposit(5);
    Account::deposit(&mut acct, 5); // the same call, written out
    println!("method syntax:   {}", acct.balance());
    println!("function syntax: {}", Account::balance(&acct));

    // A method is a function whose first parameter is the receiver, so it can be
    // stored in a variable and passed around.
    let read: fn(&Account) -> u32 = Account::balance;
    println!("through a variable: {}", read(&acct));

    let accounts = vec![Account { balance: 3 }, Account { balance: 4 }];
    let total: u32 = accounts.iter().map(Account::balance).sum();
    println!("total: {total}");
}
