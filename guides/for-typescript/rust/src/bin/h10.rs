#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    admin: bool,
}

fn main() {
    let ana = User { name: "ana".to_string(), age: 30, admin: false };
    let older = User { age: 31, ..ana }; // takes the remaining fields from ana
    println!("{} {}", ana.age, older.age);
    println!("{}", ana.name);
}
