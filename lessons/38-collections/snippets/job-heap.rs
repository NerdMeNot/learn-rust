use std::cmp::Reverse;
use std::collections::BinaryHeap;

// derive(Ord) compares the fields in the order they are written
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Job {
    priority: u8,
    name: &'static str,
}

fn main() {
    let jobs = [(2, "test"), (9, "alert"), (2, "write"), (9, "deploy")];

    let mut heap = BinaryHeap::new();
    for (priority, name) in jobs {
        heap.push(Job { priority, name });
    }
    println!("derive(Ord):");
    while let Some(Job { priority, name }) = heap.pop() {
        println!("  {priority} {name}");
    }

    // Arrival order breaks the tie if the item carries an arrival number. Reverse makes
    // the earlier arrival the greater one, so it comes out first.
    let mut fifo = BinaryHeap::new();
    for (arrival, (priority, name)) in jobs.into_iter().enumerate() {
        fifo.push((priority, Reverse(arrival), name));
    }
    println!("with an arrival number:");
    while let Some((priority, _, name)) = fifo.pop() {
        println!("  {priority} {name}");
    }
}
