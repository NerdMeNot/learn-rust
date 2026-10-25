use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::mem::size_of;

fn main() {
    let name = String::from("ana");
    let name_bytes = name.as_ptr();
    println!(
        "name before: len {}, capacity {}",
        name.len(),
        name.capacity()
    );

    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(name, 30);

    let (key, value) = ages.iter().next().unwrap();
    println!(
        "key in the map: {key}, len {}, capacity {}",
        key.len(),
        key.capacity()
    );
    println!(
        "key bytes at the same address as name's bytes: {}",
        key.as_ptr() == name_bytes
    );
    println!("value in the map: {value}");

    println!("size_of String: {}", size_of::<String>());
    println!("size_of (String, u32): {}", size_of::<(String, u32)>());
    println!(
        "size_of RandomState (the hasher): {}",
        size_of::<RandomState>()
    );
    println!(
        "size_of HashMap<String, u32>: {}",
        size_of::<HashMap<String, u32>>()
    );
}
