mod models;
mod pricing;

use models::Order;

fn main() {
    let order = Order::new("ana", 3, 1250);
    println!("{}", order.summary());
    println!("total after discounts: {}", pricing::total(&order));
    let gross = order.unit_cents * order.quantity as u64;
    println!("discount alone: {}", pricing::discounts::bulk(order.quantity, gross));
}
