// Backs the claim that `self`, `&self` and `&mut self` are shorthand for typed parameters.
struct Account {
    balance: u32,
}

impl Account {
    fn read(self: &Self) -> u32 {
        self.balance
    }
    fn change(self: &mut Self) {
        self.balance += 1;
    }
    fn consume(self: Self) -> u32 {
        self.balance
    }
}

fn main() {
    let mut acct = Account { balance: 1 };
    acct.change();
    assert_eq!(acct.read(), 2);
    assert_eq!(acct.consume(), 2);
    println!("checks passed");
}
