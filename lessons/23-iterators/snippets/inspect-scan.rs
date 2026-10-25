fn main() {
    // inspect runs a closure on each item as it passes, and changes nothing.
    // It is for debugging a chain without taking it apart.
    let total: i32 = [1, 2, 3, 4]
        .iter()
        .inspect(|n| println!("  before filter: {n}"))
        .filter(|&&n| n % 2 == 0)
        .inspect(|n| println!("  after filter: {n}"))
        .sum();
    println!("total: {total}");

    // scan is map with a running state: here, a running balance.
    let changes = [100, -30, -50, 20];
    let balances: Vec<i32> = changes
        .iter()
        .scan(0, |balance, change| {
            *balance += change;
            Some(*balance) // return None to stop early
        })
        .collect();
    println!("scan balances: {balances:?}");
}
