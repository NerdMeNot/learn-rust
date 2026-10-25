use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    for n in [3, 8, 1, 5] {
        heap.push(Reverse(n));
    }
    let Reverse(first) = heap.pop().unwrap();
    println!("{first}");
    println!("{:?}", heap.peek());
    println!("{:?}", heap.into_sorted_vec());
}
