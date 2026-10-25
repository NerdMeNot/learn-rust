//! A cache that asks a function for values it does not have yet.

use std::collections::HashMap;

pub struct Cache<F: Fn(&str) -> String> {
    fetch: F,
    items: HashMap<String, String>,
}

impl<F: Fn(&str) -> String> Cache<F> {
    pub fn new(fetch: F) -> Self {
        Cache {
            fetch,
            items: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> &str {
        self.items
            .entry(key.to_string())
            .or_insert_with(|| (self.fetch)(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // The fake fetcher is a closure that records what it was asked for.
    #[test]
    fn fetches_each_key_once() {
        let asked = RefCell::new(Vec::new());
        let mut cache = Cache::new(|key| {
            asked.borrow_mut().push(key.to_string());
            format!("value of {key}")
        });
        assert_eq!(cache.get("a"), "value of a");
        assert_eq!(cache.get("a"), "value of a");
        assert_eq!(cache.get("b"), "value of b");
        assert_eq!(*asked.borrow(), ["a", "b"]);
    }
}
