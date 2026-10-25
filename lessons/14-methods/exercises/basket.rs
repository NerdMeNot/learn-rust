struct Basket {
    items: Vec<String>,
}

impl Basket {
    /// An empty basket.
    fn new() -> Self {
        todo!("build a Basket whose items field is Vec::new()")
    }

    /// How many items are in the basket. It only reads.
    fn count(&self) -> usize {
        todo!("self.items.len()")
    }

    /// Adds the item with spaces trimmed off both ends and returns true.
    /// A blank item (empty, or only spaces) is not added: return false.
    fn add(&mut self, item: &str) -> bool {
        todo!("trim() the item; if it is empty return false, else push a String")
    }

    /// Uses the basket up and returns the items joined with ", ".
    /// An empty basket gives "(empty)".
    fn into_receipt(self) -> String {
        todo!("loop over &self.items, push_str each, and put \", \" between them")
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
