// Backs "the same closure compiles when it names the field": it captures `self.log` alone.
struct Item {
    name: String,
    count: u32,
}

struct Inventory {
    items: Vec<Item>,
    log: Vec<String>,
}

impl Inventory {
    fn check(&mut self) {
        self.items
            .iter()
            .filter(|item| item.count == 0)
            .for_each(|item| self.log.push(item.name.clone()));
    }
}

fn main() {
    let mut inv = Inventory {
        items: vec![Item {
            name: "bolt".to_string(),
            count: 0,
        }],
        log: Vec::new(),
    };
    inv.check();
    println!("log: {:?}", inv.log);
}
