use std::collections::VecDeque;

fn main() {
    let mut q: VecDeque<&str> = VecDeque::new();
    q.push_back("b");
    q.push_back("c");
    q.push_front("a");
    println!("queue: {q:?}, len {}", q.len());
    println!("front: {:?}, back: {:?}", q.front(), q.back());
    println!("index 1: {}", q[1]);
    println!("pop_front: {:?}", q.pop_front());
    println!("pop_back: {:?}", q.pop_back());
    println!("left: {q:?}");

    // iteration goes front to back, whatever the layout in memory
    let mut ring: VecDeque<i32> = (1..=5).collect();
    ring.rotate_left(2);
    println!("rotated: {ring:?}");
    // a deque is not a slice; ask for one when you need one
    ring.make_contiguous().sort();
    println!("sorted: {ring:?}");
    println!("as a slice: {:?}", ring.as_slices());
}
