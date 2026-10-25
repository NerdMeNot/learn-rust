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

    // Code in the same module may read and change private fields.
    pub fn price(item: &Item) -> u32 {
        item.price_cents
    }

    pub fn discount(item: &mut Item, cents: u32) {
        item.price_cents -= cents;
    }
}

fn main() {
    let mut item = shop::make_item("tea", 450);
    println!("name: {}", item.name);
    println!("price: {}", shop::price(&item));
    shop::discount(&mut item, 100);
    println!("price after discount: {}", shop::price(&item));
}
