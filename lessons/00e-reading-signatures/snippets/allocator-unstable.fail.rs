use std::alloc::Global;
use std::collections::HashMap;

fn main() {
    let map: HashMap<i32, i32, _, Global> = HashMap::new_in(Global);
    println!("len: {}", map.len());
}
