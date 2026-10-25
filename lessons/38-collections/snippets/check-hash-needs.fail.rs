use std::collections::HashSet;

#[derive(PartialEq, Eq)]
struct Tag(String);

fn main() {
    let mut set = HashSet::new();
    set.insert(Tag("a".into()));
}
