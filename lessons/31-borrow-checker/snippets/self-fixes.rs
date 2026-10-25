use std::mem;

struct Item {
    name: String,
    count: u32,
}

// Fix 2 needs a function that takes only the field it changes.
fn note(log: &mut Vec<String>, text: &str) {
    log.push(text.to_string());
}

struct Inventory {
    items: Vec<Item>,
    log: Vec<String>,
}

impl Inventory {
    fn note(&mut self, text: &str) {
        self.log.push(text.to_string());
    }

    // Fix 1: name the field. `items` and `log` are separate places.
    fn check_direct(&mut self) {
        for item in &self.items {
            if item.count == 0 {
                self.log.push(format!("direct: {}", item.name));
            }
        }
    }

    // Fix 2: a function that takes the one field it needs.
    fn check_field(&mut self) {
        for item in &self.items {
            if item.count == 0 {
                note(&mut self.log, &format!("field: {}", item.name));
            }
        }
    }

    // Fix 3: take the field out, call the method, put the field back.
    // While `items` is out, `self.items` is an empty Vec.
    fn check_take(&mut self) {
        let items = mem::take(&mut self.items);
        for item in &items {
            if item.count == 0 {
                self.note(&format!("take: {}", item.name));
            }
        }
        self.items = items;
    }
}

// Fix 4: make the log its own type, so a method on it borrows only the log.
struct Log {
    lines: Vec<String>,
}

impl Log {
    fn note(&mut self, text: &str) {
        self.lines.push(text.to_string());
    }
}

struct Shop {
    items: Vec<Item>,
    log: Log,
}

impl Shop {
    fn check(&mut self) {
        for item in &self.items {
            if item.count == 0 {
                self.log.note(&format!("parts: {}", item.name));
            }
        }
    }
}

fn items() -> Vec<Item> {
    vec![
        Item {
            name: "bolt".to_string(),
            count: 0,
        },
        Item {
            name: "nut".to_string(),
            count: 12,
        },
    ]
}

fn main() {
    let mut inv = Inventory {
        items: items(),
        log: Vec::new(),
    };
    inv.check_direct();
    inv.check_field();
    let buffer = inv.items.as_ptr();
    inv.check_take();
    println!("log: {:?}", inv.log);
    println!(
        "items put back: {}, same buffer: {}",
        inv.items.len(),
        inv.items.as_ptr() == buffer
    );

    let mut shop = Shop {
        items: items(),
        log: Log { lines: Vec::new() },
    };
    shop.check();
    println!("shop log: {:?}", shop.log.lines);
}
