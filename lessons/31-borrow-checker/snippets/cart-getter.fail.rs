struct Cart {
    items: Vec<String>,
}

impl Cart {
    fn items(&self) -> &Vec<String> {
        &self.items
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let cart = Cart { items: Vec::new() };
    let view = cart.items();
    view.push(String::from("tea"));
    println!("cart size: {}", cart.size());
}
