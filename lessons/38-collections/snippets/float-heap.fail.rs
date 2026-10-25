use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(2.5_f64);
    heap.push(0.5);
    println!("{:?}", heap.pop());
}
