use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from([3, 9, 4]);
    let by_key = BinaryHeap::<(u8, &str)>::new_by(|a, b| a.0.cmp(&b.0));
    heap.remove(&4);
    println!("{} {:?}", heap.contains(&9), by_key.len());
}
