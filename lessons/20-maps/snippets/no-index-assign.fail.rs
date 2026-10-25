use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock["apple"] = 3;
    println!("{}", stock.len());
}
