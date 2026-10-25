#[derive(Debug)] // lets you print it with {:?}
struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let u = User {
        name: String::from("ana"),
        age: 30,
        active: true,
    };
    println!("{} is {}, active={}", u.name, u.age, u.active);
    println!("{u:?}");
}
