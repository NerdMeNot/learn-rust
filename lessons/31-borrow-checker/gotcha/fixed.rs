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
    fn note(&mut self, text: &str) {
        self.log.push(text.to_string());
    }

    fn check(&mut self) {
        // work out what to log while `items` is borrowed, then call the method
        let mut out = Vec::new();
        for item in &self.items {
            if item.count == 0 {
                out.push(format!("out of stock: {}", item.name));
            }
        }
        for line in out {
            self.note(&line);
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
