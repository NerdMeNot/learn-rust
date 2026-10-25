// Backs "Rust refuses a push or a map remove inside a loop over a borrow". Never shown.
use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        if *x == 2 {
            v.push(99);
        }
    }
    let mut m = HashMap::from([("a", 1), ("b", 2)]);
    for (k, _) in &m {
        m.remove(k);
    }
}
