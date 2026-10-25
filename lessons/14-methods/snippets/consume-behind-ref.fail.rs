struct Account {
    owner: String,
}

impl Account {
    fn close(self) -> String {
        format!("{} closed", self.owner)
    }
}

fn close_it(acct: &Account) -> String {
    acct.close()
}

fn main() {
    let acct = Account {
        owner: String::from("ana"),
    };
    println!("{}", close_it(&acct));
}
