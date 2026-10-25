mod models; // src/models.rs is part of this program
mod pricing; // so is src/pricing.rs, plus anything it declares

use models::Order; // a shortcut: now we can write Order instead of models::Order

fn main() {
    let order = Order::new("ana", 3, 1250);
    println!("{}", order.summary());
    println!("total after discounts: {}", pricing::total(&order));
}
