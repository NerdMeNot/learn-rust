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
    match find(2) {
        Some(u) => println!("{}", u.name),
        None => println!("no such user"),
    }
}
