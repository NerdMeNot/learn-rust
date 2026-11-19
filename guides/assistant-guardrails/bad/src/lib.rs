//! A small stock-keeping library, written to show lints at work.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// How many of each item are in stock.
#[derive(Debug, Default)]
pub struct Stock {
    counts: HashMap<String, u32>,
}

impl Stock {
    /// Reads lines like `widget,3`.
    pub fn from_csv(text: &str) -> Stock {
        let mut counts = HashMap::new();
        for line in text.lines() {
            let (name, n) = line.split_once(',').unwrap();
            let n: u32 = n.trim().parse().expect("a number");
            counts.insert(name.to_string(), n);
        }
        Stock { counts }
    }

    /// Takes `n` of an item out of stock and returns how many are left.
    pub fn take(&mut self, name: &str, n: u32) -> u32 {
        let left = self.counts[name] - n;
        dbg!(left);
        self.counts.insert(name.to_string(), left);
        left
    }

    /// Puts `n` of an item back.
    pub fn restock(&mut self, _name: &str, _n: u32) {
        todo!()
    }
}

/// Gives another thread its own handle to the same stock.
pub fn share(stock: &Arc<Mutex<Stock>>) -> Arc<Mutex<Stock>> {
    stock.clone()
}

/// Writes the stock somewhere slow.
pub async fn save(stock: &Mutex<Stock>) {
    let guard = stock.lock().unwrap();
    write_out(guard.counts.len()).await;
}

async fn write_out(_items: usize) {}

#[allow(dead_code)]
fn old_parser() {}
