struct User {
    name: String,
}

fn find(id: u32) -> Option<User> {
    if id == 1 {
        Some(User { name: "ana".into() })
    } else {
        None
    }
}

fn main() {
    let u = find(2).unwrap();
    println!("{}", u.name);
}
