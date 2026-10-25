struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let u = User {
        name: String::from("ana"),
    };
    println!("{}", u.name);
}
