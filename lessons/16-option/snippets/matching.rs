struct User {
    name: &'static str,
    email: Option<&'static str>,
    age: Option<u32>,
}

fn find_user(id: u32) -> Option<User> {
    match id {
        1 => Some(User {
            name: "ana",
            email: Some("ana@example.com"),
            age: Some(31),
        }),
        2 => Some(User {
            name: "bo",
            email: None,
            age: None,
        }),
        _ => None,
    }
}

fn main() {
    for id in [1, 2, 3] {
        match find_user(id) {
            Some(User {
                name,
                email: Some(email),
                age,
            }) => {
                println!("id {id}: {name}, email {email}, age {age:?}");
            }
            Some(User {
                name,
                email: None,
                age,
            }) => {
                println!("id {id}: {name}, no email, age {age:?}");
            }
            None => println!("id {id}: no such user"),
        }
    }
}
