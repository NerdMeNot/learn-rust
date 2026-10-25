use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();
    users.insert("ada", "Ada Lovelace");
    println!("{}", users.get("bob").to_uppercase());
}
