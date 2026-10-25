// src/main.rs, without `mod pricing;`
mod models;

use models::Order;

fn main() {
    let order = Order::new("ana", 3, 1250);
    println!("{}", order.summary());
    println!("total after discounts: {}", pricing::total(&order));
}
