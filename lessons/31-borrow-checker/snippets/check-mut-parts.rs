use std::cell::Cell;
use std::collections::HashMap;

// Backs the "a `&mut` to one part" table and the `Cell` row.
struct Item {
    name: String,
    count: u32,
}

struct Counter {
    hits: Cell<u32>,
}

impl Counter {
    // Takes `&self`, yet counts: `Cell` changes through a shared reference.
    fn read(&self) -> u32 {
        self.hits.set(self.hits.get() + 1);
        5
    }
}

fn main() {
    let mut items = vec![
        Item {
            name: "a".into(),
            count: 1,
        },
        Item {
            name: "b".into(),
            count: 2,
        },
    ];
    for item in items.iter_mut() {
        item.count += 1;
    }
    assert_eq!(items[0].count, 2);
    if let Some(last) = items.last_mut() {
        last.count = 10;
    }
    assert_eq!(items[1].count, 10);
    if let Some(item) = items.iter_mut().find(|i| i.name == "a") {
        item.count = 7;
    }
    assert_eq!(items[0].count, 7);

    let mut current: Option<String> = Some("x".into());
    if let Some(text) = current.as_mut() {
        text.push('y');
    }
    if let Some(text) = current.as_deref_mut() {
        text.make_ascii_uppercase();
    }
    assert_eq!(current.as_deref(), Some("XY"));

    // Change through the `&mut`, let it go, then change the map itself.
    let mut map: HashMap<&str, Vec<u32>> = HashMap::from([("k", vec![])]);
    if let Some(list) = map.get_mut("k") {
        list.push(1);
    }
    map.insert("j", vec![2]);
    assert_eq!(map["k"], [1]);

    let counter = Counter { hits: Cell::new(0) };
    counter.read();
    counter.read();
    assert_eq!(counter.hits.get(), 2);
}
