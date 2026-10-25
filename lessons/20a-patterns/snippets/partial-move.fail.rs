struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person {
        name: String::from("Ana"),
        age: 30,
    };
    let Person { name, age } = p;
    println!("{name} is {age}");
    println!("p.name is still {}", p.name);
}
