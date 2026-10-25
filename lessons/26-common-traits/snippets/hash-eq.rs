use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

// Two tickets are the same ticket when the id matches. The note does not count.
#[derive(Hash)]
struct Derived {
    id: u32,
    note: &'static str,
}

struct ByHand {
    id: u32,
    note: &'static str,
}

impl PartialEq for Derived {
    fn eq(&self, other: &Derived) -> bool {
        self.id == other.id
    }
}
impl Eq for Derived {}

impl PartialEq for ByHand {
    fn eq(&self, other: &ByHand) -> bool {
        self.id == other.id
    }
}
impl Eq for ByHand {}

// Nothing is returned: the parts that `eq` looks at are written into the hasher.
impl Hash for ByHand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn hash_of<T: Hash>(t: &T) -> u64 {
    let mut h = DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

fn report<T: Hash + Eq>(label: &str, a: T, b: T) {
    let same_hash = hash_of(&a) == hash_of(&b);
    println!("{label}: equal {}, same hash {same_hash}", a == b);
    // A fixed hasher, so every run prints the same thing.
    let mut set: HashSet<T, BuildHasherDefault<DefaultHasher>> = HashSet::default();
    set.insert(a);
    println!("{label}: set contains the equal one: {}", set.contains(&b));
    set.insert(b);
    println!("{label}: set size {}", set.len());
}

fn main() {
    let (a, b) = (
        Derived {
            id: 7,
            note: "first",
        },
        Derived {
            id: 7,
            note: "second",
        },
    );
    println!("notes: {} and {}", a.note, b.note);
    report("derived Hash", a, b);
    let (a, b) = (
        ByHand {
            id: 7,
            note: "first",
        },
        ByHand {
            id: 7,
            note: "second",
        },
    );
    println!("notes: {} and {}", a.note, b.note);
    report("hand-written", a, b);
}
