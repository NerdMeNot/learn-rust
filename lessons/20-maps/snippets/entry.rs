use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();

    // or_insert: add 10 if the key is missing. Either way you get a &mut to the value.
    *stock.entry("apple").or_insert(10) += 1;
    *stock.entry("apple").or_insert(99) += 1;
    println!("apple: {:?}", stock.get("apple"));

    // or_insert_with: build the value only if the key is missing.
    stock.entry("pear").or_insert_with(|| {
        println!("building the pear value");
        7
    });
    stock.entry("pear").or_insert_with(|| {
        println!("building the pear value again");
        7
    });

    // or_default: the type's default value (0 for a number).
    *stock.entry("fig").or_default() += 1;

    // and_modify: change the value if the key is there, otherwise insert.
    stock.entry("fig").and_modify(|n| *n *= 10).or_insert(0);
    stock.entry("kiwi").and_modify(|n| *n *= 10).or_insert(0);

    let mut pairs: Vec<_> = stock.iter().collect();
    pairs.sort();
    println!("{pairs:?}");
}
