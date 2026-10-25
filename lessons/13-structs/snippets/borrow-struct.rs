struct Account {
    owner: String,
    balance: i64,
}

fn summary(a: &Account) -> String {
    format!("{} has {}", a.owner, a.balance)
}

fn deposit(a: &mut Account, amount: i64) {
    a.balance += amount;
}

fn owner_name(a: &Account) -> String {
    a.owner.clone()
}

fn main() {
    let mut acct = Account {
        owner: String::from("ana"),
        balance: 100,
    };
    println!("{}", summary(&acct));
    deposit(&mut acct, 50);
    println!("{}", summary(&acct));
    let name = owner_name(&acct);
    println!("copied name: {name}");
    println!("owner is still in acct: {}", acct.owner);
}
