struct Person {
    name: String,
    age: u32,
}

fn main() {
    let people = vec![Person {
        name: "ana".into(),
        age: 30,
    }];
    for p in &people {
        p.age += 1;
    }
    println!("{}", people[0].name);
}
