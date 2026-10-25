struct Basket {
    owner: String,
    items: Vec<String>,
}

/// Put `item` in the basket.
fn add(b: &mut Basket, item: &str) {
    b.items.push(item.to_string());
}

/// "ana: empty", "ana: 1 item (tea)" or "ana: 2 items (tea, jam)".
fn describe(b: &Basket) -> String {
    match b.items.len() {
        0 => format!("{}: empty", b.owner),
        1 => format!("{}: 1 item ({})", b.owner, b.items[0]),
        n => format!("{}: {n} items ({})", b.owner, b.items.join(", ")),
    }
}

/// Hand back the owner and the items. The basket is used up.
fn checkout(b: Basket) -> (String, Vec<String>) {
    (b.owner, b.items)
}

fn main() {
    let mut b = Basket {
        owner: String::from("ana"),
        items: Vec::new(),
    };
    assert_eq!(describe(&b), "ana: empty");

    add(&mut b, "tea");
    assert_eq!(describe(&b), "ana: 1 item (tea)");

    add(&mut b, "jam");
    assert_eq!(describe(&b), "ana: 2 items (tea, jam)");

    let (owner, items) = checkout(b);
    assert_eq!(owner, "ana");
    assert_eq!(items, ["tea", "jam"]);

    // An empty basket checks out with nothing.
    let (owner, items) = checkout(Basket {
        owner: String::new(),
        items: Vec::new(),
    });
    assert_eq!(owner, "");
    assert!(items.is_empty());

    println!("all checks passed");
}
