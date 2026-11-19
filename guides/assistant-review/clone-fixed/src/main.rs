fn add_item(items: &mut Vec<String>, item: &str) {
    items.push(item.to_string());
}

fn main() {
    let mut cart = vec!["apples".to_string(), "bread".to_string()];
    add_item(&mut cart, "milk");
    println!("cart: {cart:?}");
}
