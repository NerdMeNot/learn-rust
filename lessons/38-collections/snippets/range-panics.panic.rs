use std::collections::BTreeMap;

fn main() {
    let log: BTreeMap<u32, &str> = BTreeMap::from([(10, "boot"), (70, "halt")]);
    let (from, to) = (50, 20);
    println!("asking for {from}..{to}");
    let found: Vec<_> = log.range(from..to).collect();
    println!("found: {found:?}");
}
