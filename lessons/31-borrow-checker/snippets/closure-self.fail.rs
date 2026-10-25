struct Item {
    name: String,
    count: u32,
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
        self.items
            .iter()
            .filter(|item| item.count == 0)
            .for_each(|item| self.note(&item.name));
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
