fn add_item(mut items: Vec<String>, item: &str) {
    items.push(item.to_string());
}

fn main() {
    let cart = vec!["apples".to_string(), "bread".to_string()];
    add_item(cart, "milk");
    println!("cart: {cart:?}");
}
