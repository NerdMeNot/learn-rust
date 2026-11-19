// Fix 2: `summary` only reads the orders, so it borrows them.
use tally::Order;

fn summary(orders: &[Order]) -> String {
    println!("summary reads orders at {:p}", orders.as_ptr());
    let cents = tally::total_cents(orders);
    format!("{} orders, {} cents", orders.len(), cents)
}

fn main() {
    let orders = tally::parse(include_str!("../orders.csv")).expect("the sample file parses");
    println!("main holds orders at    {:p}", orders.as_ptr());
    println!("{}", summary(&orders));
    let biggest = orders
        .iter()
        .max_by_key(|o| o.cents())
        .expect("at least one order");
    println!("biggest: {}", biggest.item);
}
