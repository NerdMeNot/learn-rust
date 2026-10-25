struct Account {
    owner: String,
    balance: i64,
}

fn owner_name(a: &Account) -> String {
    a.owner
}

fn main() {
    let acct = Account {
        owner: String::from("ana"),
        balance: 100,
    };
    println!("{} ({})", owner_name(&acct), acct.balance);
}
