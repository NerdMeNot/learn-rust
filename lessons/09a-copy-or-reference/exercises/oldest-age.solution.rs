// `Person` does not derive `Clone`, so the list cannot be copied: you must borrow it.
struct Person {
    name: String,
    age: u32,
}

/// The biggest age in the list, or 0 for an empty list.
fn oldest_age(people: &Vec<Person>) -> u32 {
    let mut best = 0;
    for p in people {
        if p.age > best {
            best = p.age;
        }
    }
    best
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Ada"),
            age: 36,
        },
        Person {
            name: String::from("Grace"),
            age: 85,
        },
        Person {
            name: String::from("Linus"),
            age: 54,
        },
    ];

    assert_eq!(oldest_age(&people), 85);
    assert_eq!(people[1].age, 85);

    // The list is still ours afterwards, and Person has no Clone to copy it with.
    assert_eq!(people.len(), 3);
    assert_eq!(people[0].name, "Ada");

    // An empty list has no oldest person; the answer is 0.
    let nobody: Vec<Person> = Vec::new();
    assert_eq!(oldest_age(&nobody), 0);

    // One person.
    let one = vec![Person {
        name: String::from("Bo"),
        age: 20,
    }];
    assert_eq!(oldest_age(&one), 20);
    println!("all checks passed");
}
