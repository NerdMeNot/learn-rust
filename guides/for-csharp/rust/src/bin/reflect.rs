#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person {
        name: "Ana".to_string(),
        age: 36,
    };
    println!("{p:?}");
    println!("{} is {}", p.name, p.age);
    println!("{p:#?}");
}
