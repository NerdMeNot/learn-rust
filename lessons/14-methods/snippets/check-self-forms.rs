// Backs the table of receivers: which forms need a `let mut` variable.
struct Account {
    balance: u32,
}

impl Account {
    fn read(&self) -> u32 {
        self.balance
    }
    fn with_bonus(mut self, bonus: u32) -> Self {
        self.balance += bonus;
        self
    }
    fn change(&mut self) {
        self.balance += 1;
    }
    fn consume(self) -> u32 {
        self.balance
    }
}

fn main() {
    // a plain `let` is enough for `&self`, `mut self` and `self`
    let acct = Account { balance: 1 };
    assert_eq!(acct.read(), 1);
    let acct = acct.with_bonus(4);
    assert_eq!(acct.read(), 5);
    assert_eq!(acct.consume(), 5);
    // only `&mut self` needs `let mut`
    let mut acct = Account { balance: 1 };
    acct.change();
    assert_eq!(acct.read(), 2);
    println!("checks passed");
}
