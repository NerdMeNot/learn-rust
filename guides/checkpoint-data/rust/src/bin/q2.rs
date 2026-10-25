struct User {
    name: String,
    age: u32,
}

fn main() {
    let a = User { name: String::from("Ada"), age: 36 };
    let b = User { age: 37, ..a };
    println!("{} {}", b.name, b.age);
    println!("{}", a.age);
}
