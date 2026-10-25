#[derive(Default)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    // struct update syntax (explained in the lesson): the rest comes from Default
    let u = User {
        name: String::from("ana"),
        ..Default::default()
    };
    println!("{} {} {}", u.name, u.age, u.active);
}
