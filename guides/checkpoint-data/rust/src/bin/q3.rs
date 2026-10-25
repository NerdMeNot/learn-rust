use std::collections::BTreeMap;

fn main() {
    let mut stock: BTreeMap<&str, u32> = BTreeMap::new();
    println!("{:?}", stock.insert("pear", 4));
    println!("{:?}", stock.insert("pear", 7));
    *stock.entry("fig").or_insert(10) += 1;
    *stock.entry("fig").or_insert(10) += 1;
    stock.entry("apple").or_default();
    println!("{:?}", stock);
}
