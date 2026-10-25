use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicUsize, Ordering as Atomic};

static EQ_CALLS: AtomicUsize = AtomicUsize::new(0);

/// A word whose `cmp` ignores case but whose `==` does not: the two disagree on purpose.
#[derive(Debug, Eq)]
struct Word(&'static str);

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        EQ_CALLS.fetch_add(1, Atomic::SeqCst);
        self.0 == other.0
    }
}
impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Word {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.to_lowercase().cmp(&other.0.to_lowercase())
    }
}

fn eq_calls() -> usize {
    EQ_CALLS.load(Atomic::SeqCst)
}

fn main() {
    let mut set = BTreeSet::new();
    println!("insert Ana: {}", set.insert(Word("Ana")));
    println!("insert ana: {}", set.insert(Word("ana")));
    println!("inserted: {set:?}, `==` called {} times", eq_calls());

    let collected: BTreeSet<Word> = [Word("Ana"), Word("ana")].into_iter().collect();
    println!("collected: {collected:?}, `==` called {} times", eq_calls());
}
