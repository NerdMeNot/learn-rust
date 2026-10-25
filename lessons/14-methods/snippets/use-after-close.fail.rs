struct Account {
    owner: String,
    balance: u32,
}

impl Account {
    fn close(self) -> String {
        format!("{} closed with {}", self.owner, self.balance)
    }
}

fn main() {
    let acct = Account {
        owner: String::from("ana"),
        balance: 75,
    };
    let note = acct.close();
    println!("note: {note}");
    println!("balance: {}", acct.balance);
}
