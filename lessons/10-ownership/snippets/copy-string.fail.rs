#[derive(Clone, Copy)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let u = User {
        id: 1,
        name: String::from("ana"),
    };
    println!("{} {}", u.id, u.name);
}
