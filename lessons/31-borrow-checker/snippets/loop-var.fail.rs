struct Item {
    name: String,
}

fn main() {
    let items = vec![
        Item {
            name: String::from("a"),
        },
        Item {
            name: String::from("b"),
        },
    ];
    let mut refs: Vec<&Item> = Vec::new();
    for it in items {
        refs.push(&it);
    }
    for r in refs {
        println!("{}", r.name);
    }
}
