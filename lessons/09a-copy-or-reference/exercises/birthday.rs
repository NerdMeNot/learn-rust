struct Person {
    name: String,
    age: u32,
}

fn ages(people: &Vec<Person>) -> Vec<u32> {
    let mut out = Vec::new();
    for p in people {
        out.push(p.age);
    }
    out
}

/// Adds one to every age, in the caller's list.
fn birthday(people: &mut Vec<Person>) {
    todo!(
        "people is already a reference: loop with `people.iter_mut()`, so each `p` is a `&mut Person`"
    )
}

fn main() {
    let mut people = vec![
        Person {
            name: String::from("Ada"),
            age: 36,
        },
        Person {
            name: String::from("Linus"),
            age: 54,
        },
        Person {
            name: String::from("Grace"),
            age: 85,
        },
    ];
    let mut others = vec![Person {
        name: String::from("Bo"),
        age: 20,
    }];

    birthday(&mut people);
    assert_eq!(ages(&people), [37, 55, 86]);
    assert_eq!(people[0].name, "Ada");

    // Only the list you pass is changed.
    assert_eq!(ages(&others), [20]);

    // An empty list is fine.
    let mut nobody: Vec<Person> = Vec::new();
    birthday(&mut nobody);
    assert!(nobody.is_empty());

    birthday(&mut others);
    assert_eq!(ages(&others), [21]);
    println!("all checks passed");
}
