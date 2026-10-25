use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    set.insert(2.5_f64);
    println!("{}", set.len());
}
