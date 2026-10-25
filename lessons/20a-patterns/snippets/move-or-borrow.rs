struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person {
        name: String::from("Ana"),
        age: 30,
    };

    // `ref` borrows the field instead of moving it out
    let Person { ref name, age } = p;
    println!("ref: {name} is {age}");

    // matching on a reference borrows too, without `ref`; here `age` is a &u32 as well
    let Person { name, age } = &p;
    println!("&p: {name} is {age}");

    // p is still whole, so it can be used or moved afterwards
    let moved = p;
    println!("moved: {}", moved.name);
}
