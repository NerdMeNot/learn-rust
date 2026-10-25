#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn person(name: &str, age: u32) -> Person {
    Person {
        name: name.to_string(),
        age,
    }
}

fn main() {
    let mut people = vec![person("Cy", 40), person("Ana", 31), person("Bo", 25)];

    people.sort_by_key(|p| p.age);
    println!(
        "by age: {:?}",
        people.iter().map(|p| &p.name).collect::<Vec<_>>()
    );

    people.sort_by(|a, b| a.name.cmp(&b.name));
    println!(
        "by name: {:?}",
        people.iter().map(|p| &p.name).collect::<Vec<_>>()
    );

    let min_age = 30;
    people.retain(|p| p.age >= min_age);
    println!(
        "aged {min_age}+: {:?}",
        people.iter().map(|p| &p.name).collect::<Vec<_>>()
    );

    let total: u32 = people.iter().map(|p| p.age).sum();
    println!("total age: {total}");
}
