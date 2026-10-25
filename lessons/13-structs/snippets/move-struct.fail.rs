struct Order {
    customer: String,
    items: Vec<String>,
}

fn ship(order: Order) {
    println!("shipping {} items to {}", order.items.len(), order.customer);
}

fn main() {
    let order = Order {
        customer: String::from("ana"),
        items: vec![String::from("tea")],
    };
    ship(order);
    println!("customer: {}", order.customer);
}
