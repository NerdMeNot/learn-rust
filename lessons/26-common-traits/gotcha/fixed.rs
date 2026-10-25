use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Version {
    major: u32,
    minor: u32,
}

fn main() {
    let mut notes = HashMap::new();
    notes.insert(Version { major: 1, minor: 2 }, "stable");
    let found = notes.get(&Version { major: 1, minor: 2 });
    println!("found: {found:?}");
}
