//! A small stock-keeping library, written to show lints at work.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, PoisonError};

/// How many of each item are in stock.
#[derive(Debug, Default)]
pub struct Stock {
    counts: HashMap<String, u32>,
}

/// What can go wrong.
#[derive(Debug, PartialEq)]
pub enum StockError {
    /// A line was not `name,count`.
    BadLine(String),
    /// There are fewer items than were asked for (or none at all).
    NotEnough { name: String, have: u32, want: u32 },
    /// Another thread panicked while holding the lock.
    Poisoned,
}

impl fmt::Display for StockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StockError::BadLine(line) => write!(f, "bad line: {line:?}"),
            StockError::NotEnough { name, have, want } => {
                write!(f, "wanted {want} {name}, have {have}")
            }
            StockError::Poisoned => write!(f, "the stock lock is poisoned"),
        }
    }
}

impl std::error::Error for StockError {}

impl<T> From<PoisonError<T>> for StockError {
    fn from(_: PoisonError<T>) -> Self {
        StockError::Poisoned
    }
}

/// The stock a new shop starts with.
const STARTER: &str = "widget,10\ngadget,4";

impl Stock {
    /// Reads lines like `widget,3`.
    ///
    /// ```
    /// let stock = stock::Stock::from_csv("widget,3\ngadget,1")?;
    /// assert_eq!(stock.count("widget"), 3);
    /// # Ok::<(), stock::StockError>(())
    /// ```
    pub fn from_csv(text: &str) -> Result<Stock, StockError> {
        let mut counts = HashMap::new();
        for line in text.lines() {
            let bad = || StockError::BadLine(line.to_string());
            let (name, n) = line.split_once(',').ok_or_else(bad)?;
            let n: u32 = n.trim().parse().map_err(|_| bad())?;
            counts.insert(name.to_string(), n);
        }
        Ok(Stock { counts })
    }

    /// The stock every new shop starts with.
    #[expect(
        clippy::unwrap_used,
        reason = "STARTER is a constant, and a test parses it"
    )]
    pub fn starter() -> Stock {
        Stock::from_csv(STARTER).unwrap()
    }

    /// How many of an item there are (0 if it is unknown).
    pub fn count(&self, name: &str) -> u32 {
        self.counts.get(name).copied().unwrap_or(0)
    }

    /// Takes `n` of an item out of stock and returns how many are left.
    pub fn take(&mut self, name: &str, n: u32) -> Result<u32, StockError> {
        let have = self.count(name);
        let left = have.checked_sub(n).ok_or(StockError::NotEnough {
            name: name.to_string(),
            have,
            want: n,
        })?;
        self.counts.insert(name.to_string(), left);
        Ok(left)
    }

    /// Puts `n` of an item back.
    pub fn restock(&mut self, name: &str, n: u32) {
        *self.counts.entry(name.to_string()).or_insert(0) += n;
    }
}

/// Gives another thread its own handle to the same stock.
pub fn share(stock: &Arc<Mutex<Stock>>) -> Arc<Mutex<Stock>> {
    Arc::clone(stock)
}

/// Writes the stock somewhere slow.
pub async fn save(stock: &Mutex<Stock>) -> Result<(), StockError> {
    let items = stock.lock()?.counts.len(); // the guard is dropped at the end of this line
    write_out(items).await;
    Ok(())
}

async fn write_out(_items: usize) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_parses() {
        assert_eq!(Stock::starter().count("widget"), 10);
    }

    #[test]
    fn take_too_many_is_an_error() {
        let mut stock = Stock::from_csv("widget,2").unwrap();
        assert_eq!(stock.take("widget", 1), Ok(1));
        let err = stock.take("widget", 5).unwrap_err();
        assert_eq!(err.to_string(), "wanted 5 widget, have 1");
    }

    #[test]
    fn bad_line_is_an_error() {
        let err = Stock::from_csv("widget").unwrap_err();
        assert_eq!(err, StockError::BadLine("widget".to_string()));
    }
}
