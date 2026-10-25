mod models;
mod pricing;

// No `use` line: the full path does the same job.
fn main() {
    let order = models::Order::new("ana", 3, 1250);
    println!("{}", order.summary());
    println!("total after discounts: {}", pricing::total(&order));
}
