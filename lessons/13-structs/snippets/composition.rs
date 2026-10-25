#[derive(Debug, Clone)]
struct User {
    name: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
struct Admin {
    user: User,
    level: u8,
}

fn main() {
    let a = Admin {
        user: User {
            name: String::from("ana"),
            tags: vec![String::from("staff")],
        },
        level: 2,
    };
    println!("field through the inner struct: {}", a.user.name);
    println!("level: {}", a.level);

    let mut b = a.clone(); // `let b = a;` would move `a`
    b.user.name = String::from("bob");
    b.user.tags[0] = String::from("guest");
    println!("a: {} {:?}", a.user.name, a.user.tags);
    println!("b: {} {:?}", b.user.name, b.user.tags);
}
