#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicUsize, Ordering};

static CLONES: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, PartialEq)]
struct Entry {
    memo: String,
    amount: i64,
}

// Counts every clone, so a "fix" that copies an entry shows up in the checks.
impl Clone for Entry {
    fn clone(&self) -> Entry {
        CLONES.fetch_add(1, Ordering::SeqCst);
        Entry {
            memo: self.memo.clone(),
            amount: self.amount,
        }
    }
}

struct Ledger {
    entries: Vec<Entry>,
    balance: i64,
    limit: i64,
    journal: Vec<String>,
}

impl Ledger {
    fn new(limit: i64, entries: &[(&str, i64)]) -> Ledger {
        Ledger {
            entries: entries
                .iter()
                .map(|(memo, amount)| Entry {
                    memo: memo.to_string(),
                    amount: *amount,
                })
                .collect(),
            balance: 0,
            limit,
            journal: Vec::new(),
        }
    }

    // Applies one entry, unless the balance would pass `limit`. Leave this as it is.
    fn apply(&mut self, entry: &Entry) -> Result<(), String> {
        if self.balance + entry.amount > self.limit {
            return Err(format!("{} would pass the limit", entry.memo));
        }
        self.balance += entry.amount;
        self.journal.push(format!("applied {}", entry.memo));
        Ok(())
    }

    // Apply every entry in `entries`, in order, and stop at the first `Err`, which you
    // return. The entries stay in `entries` either way, in the same buffer.
    fn settle(&mut self) -> Result<(), String> {
        let entries = std::mem::take(&mut self.entries);
        let mut result = Ok(());
        for entry in &entries {
            if let Err(message) = self.apply(entry) {
                result = Err(message);
                break;
            }
        }
        self.entries = entries;
        result
    }
}

fn main() {
    let mut ledger = Ledger::new(100, &[("rent", 60), ("food", 30), ("fun", 20), ("tea", 5)]);
    let buffer = ledger.entries.as_ptr();

    // The third entry would pass the limit: stop there, keep every entry.
    assert_eq!(ledger.settle(), Err("fun would pass the limit".to_string()));
    assert_eq!(ledger.balance, 90);
    assert_eq!(ledger.journal, ["applied rent", "applied food"]);
    assert_eq!(
        ledger.entries.len(),
        4,
        "an early return must not lose the entries"
    );
    assert_eq!(
        ledger.entries.as_ptr(),
        buffer,
        "the entries were copied, not put back"
    );
    assert_eq!(ledger.entries[3].memo, "tea");

    // With room, every entry is applied, in order.
    let mut roomy = Ledger::new(1000, &[("a", 1), ("b", 2), ("c", 3)]);
    assert_eq!(roomy.settle(), Ok(()));
    assert_eq!(roomy.balance, 6);
    assert_eq!(roomy.journal, ["applied a", "applied b", "applied c"]);
    assert_eq!(roomy.entries.len(), 3);

    let mut empty = Ledger::new(10, &[]);
    assert_eq!(empty.settle(), Ok(()));
    assert!(empty.journal.is_empty());

    assert_eq!(CLONES.load(Ordering::SeqCst), 0, "do not clone the entries");
    println!("all checks passed");
}
