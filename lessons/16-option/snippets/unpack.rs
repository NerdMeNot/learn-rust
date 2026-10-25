struct User {
    name: &'static str,
    age: Option<u32>,
}

fn find_user(id: u32) -> Option<User> {
    match id {
        1 => Some(User {
            name: "ana",
            age: Some(31),
        }),
        2 => Some(User {
            name: "bo",
            age: None,
        }),
        _ => None,
    }
}

// if let: run some code only for Some, and ignore None.
fn print_age(id: u32) {
    if let Some(User { age: Some(age), .. }) = find_user(id) {
        println!("id {id}: age {age}");
    } else {
        println!("id {id}: age unknown");
    }
}

// let else: unwrap or leave. The happy path stays unindented.
fn greeting(id: u32) -> String {
    let Some(user) = find_user(id) else {
        return format!("id {id}: nobody home");
    };
    format!("id {id}: hello, {}", user.name)
}

fn main() {
    print_age(1);
    print_age(2);
    print_age(3);
    println!("{}", greeting(1));
    println!("{}", greeting(3));
}
