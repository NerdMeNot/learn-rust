use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A float that can be ordered: `total_cmp` puts every value, NaN included, in one order.
#[derive(Debug, PartialEq)]
struct Score(f64);

impl Eq for Score {}
impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

fn main() {
    let mut heap = BinaryHeap::new();
    for x in [2.5, 0.5, 9.0, -1.0] {
        heap.push(Score(x));
    }
    println!("largest: {:?}", heap.pop());
    println!("next: {:?}", heap.pop());
}
