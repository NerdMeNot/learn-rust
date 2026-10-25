/// One line of a receipt, in three columns separated by `|`:
///   the name, left-aligned in a column 10 wide;
///   the quantity, right-aligned in a column 3 wide;
///   the price with 2 decimals, right-aligned in a column 8 wide.
fn row(name: &str, qty: u32, price: f64) -> String {
    format!("{:<10}|{:>3}|{:>8.2}", name, qty, price)
}

fn main() {
    assert_eq!(row("apple", 3, 1.5), "apple     |  3|    1.50");
    assert_eq!(row("melon", 12, 1234.5), "melon     | 12| 1234.50");
    // The case to watch: a value wider than its column is not cut off.
    assert_eq!(
        row("watermelon-xl", 1000, 2.0),
        "watermelon-xl|1000|    2.00"
    );
    // Width counts characters, not bytes: "crème" is 5 characters (6 bytes).
    assert_eq!(row("crème", 1, 0.5), "crème     |  1|    0.50");
    println!("all checks passed");
}
