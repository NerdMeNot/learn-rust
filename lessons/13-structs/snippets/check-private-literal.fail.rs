mod shop {
    pub struct Item {
        pub name: String,
        price_cents: u32,
    }
}

fn main() {
    let item = shop::Item {
        name: String::from("tea"),
        price_cents: 450,
    };
    println!("{}", item.name);
}
