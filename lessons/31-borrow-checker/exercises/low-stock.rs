#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicUsize, Ordering};

static CLONES: AtomicUsize = AtomicUsize::new(0);

struct Item {
    name: String,
    count: u32,
}

// Counts every clone, so a "fix" that copies the items shows up in the checks.
impl Clone for Item {
    fn clone(&self) -> Item {
        CLONES.fetch_add(1, Ordering::SeqCst);
        Item {
            name: self.name.clone(),
            count: self.count,
        }
    }
}

struct Shop {
    items: Vec<Item>,
    log: Vec<String>,
    notes_made: usize,
}

impl Shop {
    fn new(items: &[(&str, u32)]) -> Shop {
        let items = items
            .iter()
            .map(|(name, count)| Item {
                name: name.to_string(),
                count: *count,
            })
            .collect();
        Shop {
            items,
            log: Vec::new(),
            notes_made: 0,
        }
    }

    // Every log line goes through here, and `note` numbers it.
    fn note(&mut self, text: &str) {
        self.notes_made += 1;
        self.log.push(format!("#{} {}", self.notes_made, text));
    }

    // For each item with a count below 5, in order, call `self.note` with
    // "low: NAME (COUNT)". Call `note` itself, and do not copy the items.
    fn flag_low(&mut self) {
        todo!("write the loop you would write anywhere else, read the error, then change it")
    }
}

fn main() {
    let mut shop = Shop::new(&[("bolt", 0), ("nut", 12), ("washer", 4), ("screw", 5)]);
    shop.flag_low();
    assert_eq!(
        shop.log,
        ["#1 low: bolt (0)", "#2 low: washer (4)"],
        "every line must come from `note`, in order"
    );
    assert_eq!(shop.notes_made, 2);
    assert_eq!(shop.items.len(), 4);
    assert_eq!(CLONES.load(Ordering::SeqCst), 0, "do not clone the items");

    let mut empty = Shop::new(&[]);
    empty.flag_low();
    assert!(empty.log.is_empty());

    shop.flag_low();
    assert_eq!(shop.log.len(), 4, "calling it again adds to the log");
    assert_eq!(shop.log[3], "#4 low: washer (4)");
    println!("all checks passed");
}
