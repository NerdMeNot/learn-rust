struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let u = User { name: "ana".into() };
    println!("{} {} {}", u.name, u.age, u.active);
}
