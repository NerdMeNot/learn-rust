struct Cart {
    items: Vec<String>,
}

impl Cart {
    // A read-only view: the caller cannot push through it.
    fn items(&self) -> &[String] {
        &self.items
    }

    // Changing the cart goes through a method that can keep its own rules.
    fn add(&mut self, item: &str) {
        if !self.items.iter().any(|i| i == item) {
            self.items.push(item.to_string());
        }
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut cart = Cart { items: Vec::new() };
    cart.add("tea");
    cart.add("tea");
    println!("cart size: {}", cart.size());

    // A copy the caller may change freely costs a copy.
    let mut mine = cart.items().to_vec();
    mine.push(String::from("milk"));
    println!("mine: {}, cart: {}", mine.len(), cart.size());
}
