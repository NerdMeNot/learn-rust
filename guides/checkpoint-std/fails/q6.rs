use std::collections::BinaryHeap;

fn main() {
    let readings: Vec<f64> = vec![2.5, 0.5, 1.5];
    let heap = BinaryHeap::from(readings);
    println!("{}", heap.len());
}
