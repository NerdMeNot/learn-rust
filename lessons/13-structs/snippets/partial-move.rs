struct Order {
    customer: String,
    items: Vec<String>,
    id: u32,
}

fn main() {
    let order = Order {
        customer: String::from("ana"),
        items: vec![String::from("tea")],
        id: 7,
    };
    let before = order.customer.as_ptr();

    let customer = order.customer; // moves one field out
    println!("customer: {customer}");
    println!("same heap bytes: {}", customer.as_ptr() == before);

    // The other fields are still there.
    println!("id: {}", order.id);
    println!("items: {}", order.items.len());
}
