struct Order {
    customer: String,
    items: Vec<String>,
}

fn count(order: &Order) -> usize {
    order.items.len()
}

fn main() {
    let order = Order {
        customer: String::from("ana"),
        items: vec![String::from("tea")],
    };
    let customer = order.customer;
    println!("customer: {customer}");
    println!("items: {}", count(&order));
}
