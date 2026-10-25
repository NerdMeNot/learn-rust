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
    match find_user(1) {
        Some(u) => println!("found {}", u.name),
        None => println!("no user"),
    }

    // if let: the short form when you only care about Some
    if let Some(u) = find_user(2) {
        println!("found {}", u.name);
    } else {
        println!("no user");
    }
}
