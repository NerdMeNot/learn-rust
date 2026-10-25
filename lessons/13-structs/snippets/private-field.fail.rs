mod shop {
    pub struct Item {
        pub name: String,
        price_cents: u32,
    }

    pub fn make_item(name: &str, price_cents: u32) -> Item {
        Item {
            name: name.to_string(),
            price_cents,
        }
    }
}

fn main() {
    let item = shop::make_item("tea", 450);
    println!("name: {}", item.name);
    println!("price: {}", item.price_cents);
}
