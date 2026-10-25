use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::from([("bolt", 0), ("nut", 12), ("washer", 0)]);
    for (name, count) in &stock {
        if *count == 0 {
            stock.remove(name);
        }
    }
    println!("{} items left", stock.len());
}
