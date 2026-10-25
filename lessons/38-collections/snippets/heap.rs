use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from([3, 9, 4, 1]);
    println!("peek: {:?}, len {}", heap.peek(), heap.len());
    heap.push(7);
    print!("pop order:");
    while let Some(n) = heap.pop() {
        print!(" {n}");
    }
    println!();

    // Reverse flips the order, so the smallest comes out first
    let mut small: BinaryHeap<Reverse<i32>> = [3, 9, 4, 1].into_iter().map(Reverse).collect();
    println!("smallest: {:?}", small.pop().map(|Reverse(n)| n));

    // tuples compare field by field: put the priority first
    let mut jobs = BinaryHeap::new();
    jobs.push((2, "write"));
    jobs.push((5, "deploy"));
    jobs.push((2, "test"));
    println!("jobs, smallest first: {:?}", jobs.into_sorted_vec());

    // peek_mut lets you change the largest item in place
    let mut h = BinaryHeap::from([5, 8, 2]);
    if let Some(mut top) = h.peek_mut() {
        *top = 1;
    }
    println!("after lowering the top: peek {:?}", h.peek());
}
