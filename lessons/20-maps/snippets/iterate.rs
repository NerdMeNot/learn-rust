use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::from([("apple", 3), ("pear", 5), ("fig", 1)]);

    let mut total = 0;
    for (_name, count) in &stock {
        total += count;
    }
    println!("total: {total}");

    for count in stock.values_mut() {
        *count *= 2;
    }
    for (_name, count) in &mut stock {
        *count += 1;
    }

    // The order of a HashMap is arbitrary, so sort the keys to print in a fixed order.
    let mut names: Vec<_> = stock.keys().collect();
    names.sort();
    for name in names {
        println!("{name}: {}", stock[name]);
    }
}
