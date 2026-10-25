use std::collections::HashSet;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

static EQ_CALLS: AtomicUsize = AtomicUsize::new(0);

/// An id that counts every equality test made with it.
#[derive(Debug, Clone, Hash, Eq)]
pub struct Id(u32);

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        EQ_CALLS.fetch_add(1, Ordering::SeqCst);
        self.0 == other.0
    }
}

/// The distinct items, each once, in the order they first appear.
/// `first_seen(&[3, 1, 3, 2, 1])` is `[3, 1, 2]`. Sorting would lose the order, and
/// asking a `Vec` "do you contain it?" for every item compares each item with many others.
/// A `HashSet` finds repeats quickly, but it forgets the order it was filled in.
pub fn first_seen<T: Hash + Eq + Clone>(items: &[T]) -> Vec<T> {
    todo!("one collection remembers what has been seen; another holds the answer in order")
}

fn main() {
    assert_eq!(first_seen(&[3, 1, 3, 2, 1]), [3, 1, 2]);
    assert_eq!(first_seen(&["b", "a", "b"]), ["b", "a"]);
    assert_eq!(first_seen::<i32>(&[]), []);
    assert_eq!(first_seen(&[7]), [7]);
    assert_eq!(first_seen(&[5, 5, 5]), [5]);

    let ids: Vec<Id> = (0..4_000u32).map(|i| Id((i * 31) % 1_000)).collect();
    EQ_CALLS.store(0, Ordering::SeqCst);
    let unique = first_seen(&ids);
    let calls = EQ_CALLS.load(Ordering::SeqCst);
    assert_eq!(unique.len(), 1_000);
    assert_eq!(unique[0], Id(0));
    assert_eq!(unique[1], Id(31));
    assert!(
        calls < 20_000,
        "made {calls} equality tests for 4,000 items: use a set, not a list"
    );
    println!("all checks passed");
}
