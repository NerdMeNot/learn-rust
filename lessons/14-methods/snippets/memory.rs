use std::mem::size_of;

struct Account {
    owner: String,
    balance: u32,
}

impl Account {
    fn itself(&self) -> &Account {
        self
    }

    fn balance(&self) -> u32 {
        self.balance
    }

    fn text_address(&self) -> *const u8 {
        self.owner.as_ptr()
    }

    fn close(self) -> *const u8 {
        self.owner.as_ptr()
    }
}

fn main() {
    let acct = Account {
        owner: String::from("ana"),
        balance: 75,
    };
    println!("size of Account: {} bytes", size_of::<Account>());
    println!("size of String:  {} bytes", size_of::<String>());
    println!("size of u32:     {} bytes", size_of::<u32>());
    println!(
        "owner: {} bytes of text, capacity {}",
        acct.owner.len(),
        acct.owner.capacity()
    );
    println!("balance: {}", acct.balance());
    println!(
        "&self is the caller's variable: {}",
        std::ptr::eq(acct.itself(), &acct)
    );

    let before = acct.text_address();
    let after = acct.close();
    println!("same text buffer after close: {}", before == after);
}
