use std::collections::hash_map::RandomState;
// BuildHasher is the trait that provides hash_one: "hash this key with my seed".
use std::hash::BuildHasher;

fn main() {
    let first = RandomState::new();
    let second = RandomState::new();

    println!(
        "same state, same key: {}",
        first.hash_one("ana") == first.hash_one("ana")
    );
    println!(
        "same state, other key: {}",
        first.hash_one("ana") == first.hash_one("bo")
    );
    println!(
        "other state, same key: {}",
        first.hash_one("ana") == second.hash_one("ana")
    );
}
