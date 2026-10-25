struct Item {
    name: String,
    count: u32,
}

impl Item {
    fn new(name: &str, count: u32) -> Item {
        Item {
            name: name.to_string(),
            count,
        }
    }
}

struct Inventory {
    items: Vec<Item>,
    log: Vec<String>,
}

impl Inventory {
    fn check(&mut self) {
        for item in &self.items {
            if item.count == 0 {
                // `items` and `log` are two fields: reading one while writing the other is fine
                self.log.push(format!("out of stock: {}", item.name));
            }
        }
    }
}

fn main() {
    let mut inv = Inventory {
        items: vec![
            Item::new("bolt", 0),
            Item::new("nut", 12),
            Item::new("washer", 0),
        ],
        log: Vec::new(),
    };
    inv.check();
    println!("log: {:?}", inv.log);
}
