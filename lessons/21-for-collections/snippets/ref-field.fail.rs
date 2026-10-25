struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut people = vec![Person {
        name: String::from("ana"),
        age: 30,
    }];

    for p in &people {
        p.age += 1;
        println!("{} is {}", p.name, p.age);
    }
    people.clear();
}
