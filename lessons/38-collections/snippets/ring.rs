use std::collections::VecDeque;

fn main() {
    let mut q: VecDeque<u32> = VecDeque::with_capacity(4);
    println!("capacity: {}", q.capacity());
    println!("bytes in a VecDeque<u32>: {}", size_of::<VecDeque<u32>>());
    println!("bytes in a Vec<u32>: {}", size_of::<Vec<u32>>());
    q.extend([1, 2, 3, 4]);
    println!("full:          {:?}", q.as_slices());
    q.pop_front();
    q.pop_front();
    println!("two popped:    {:?}", q.as_slices());
    q.push_back(5);
    q.push_back(6);
    // The new items did not move the old ones: they wrapped round to the free front.
    println!("two pushed:    {:?}", q.as_slices());
    println!("iteration:     {:?}", q.iter().collect::<Vec<_>>());
    println!("capacity still {}", q.capacity());
    q.make_contiguous();
    println!("contiguous:    {:?}", q.as_slices());
}
