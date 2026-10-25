struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut people = vec![
        Person {
            name: "ana".into(),
            age: 30,
        },
        Person {
            name: "bo".into(),
            age: 41,
        },
    ];
    for p in &mut people {
        p.age += 1; // p is a &mut Person: it reaches the element itself
    }
    for p in &people {
        println!("{} {}", p.name, p.age);
    }
}
