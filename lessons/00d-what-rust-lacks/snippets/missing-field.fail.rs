struct User {
    name: String,
    age: u32,
}

fn main() {
    let u = User {
        name: String::from("Ada"),
    };
    println!("{}", u.name);
}
