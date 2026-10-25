struct User {
    name: String,
}

fn main() {
    let mut u = User {
        name: String::from("ana"),
    };
    u.nickname = String::from("an");
    println!("{}", u.name);
}
