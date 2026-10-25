struct Person {
    name: String,
}

fn main() {
    let mut people = vec![
        Person {
            name: String::from("Cy"),
        },
        Person {
            name: String::from("Ana"),
        },
    ];
    people.sort_by_key(|p| &p.name);
    println!("{}", people[0].name);
}
