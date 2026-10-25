use std::any::type_name_of_val;
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("apple", 3);
    stock.insert("pear", 0);

    for pair in &stock {
        println!("for pair in &stock: pair is {}", type_name_of_val(&pair));
        break;
    }

    for (_, count) in &mut stock {
        *count += 10; // values can be changed; keys cannot
    }
    let mut totals: Vec<_> = stock.values().collect();
    totals.sort();
    println!("counts after +10: {totals:?}");

    for (i, c) in "héllo".char_indices() {
        print!("{i}:{c} ");
    }
    println!();
}
