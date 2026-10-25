struct Basket {
    items: Vec<String>,
}

impl Basket {
    /// An empty basket.
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// How many items are in the basket. It only reads.
    fn count(&self) -> usize {
        self.items.len()
    }

    /// Adds the item with spaces trimmed off both ends and returns true.
    /// A blank item (empty, or only spaces) is not added: return false.
    fn add(&mut self, item: &str) -> bool {
        let item = item.trim();
        if item.is_empty() {
            return false;
        }
        self.items.push(item.to_string());
        true
    }

    /// Uses the basket up and returns the items joined with ", ".
    /// An empty basket gives "(empty)".
    fn into_receipt(self) -> String {
        if self.items.is_empty() {
            return String::from("(empty)");
        }
        let mut receipt = String::new();
        for item in &self.items {
            if !receipt.is_empty() {
                receipt.push_str(", ");
            }
            receipt.push_str(item);
        }
        receipt
    }
}

fn main() {
    let mut basket = Basket::new();
    assert_eq!(basket.count(), 0);
    assert!(basket.add("café"));
    assert!(basket.add("  tea  "));
    assert!(!basket.add("   "), "a blank item is not added");
    assert!(!basket.add(""), "an empty item is not added");
    assert_eq!(basket.count(), 2);
    assert_eq!(basket.into_receipt(), "café, tea");

    let empty = Basket::new();
    assert_eq!(empty.into_receipt(), "(empty)");

    println!("all checks passed");
}
