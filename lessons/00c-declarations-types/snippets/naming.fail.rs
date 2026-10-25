#![deny(nonstandard_style)]

const max_users: u32 = 10;

struct user_account {
    UserName: String,
}

fn main() {
    let myCount = 3;
    let account = user_account {
        UserName: String::from("ada"),
    };
    println!("{myCount} {max_users} {}", account.UserName);
}
