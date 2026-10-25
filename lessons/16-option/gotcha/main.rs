struct User {
    name: String,
}

fn find_user(id: u32) -> Option<User> {
    if id == 1 {
        Some(User {
            name: String::from("ana"),
        })
    } else {
        None
    }
}

fn main() {
    let u = find_user(2);
    println!("{}", u.name);
}
