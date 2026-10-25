// Backs smaller sentences on the page.
use std::cell::Cell;
use std::collections::HashMap;

// an unbounded T still has what every type has: `into` (to itself) compiles
fn same<T>(t: T) -> T {
    t.into()
}

// `T: PartialOrd` is shorthand for `T: PartialOrd<T>`
fn bigger<T: PartialOrd<T>>(a: T, b: T) -> bool {
    a > b
}

struct Store(HashMap<String, i32>);

impl Store {
    // two reference parameters, one of them `&self`: no `'a` needed
    fn get(&self, key: &str) -> &i32 {
        &self.0[key]
    }
}

// holding a `&mut` reference is enough to call `entry`: no `let mut` here
fn bump(m: &mut HashMap<String, i32>) {
    *m.entry("a".to_string()).or_insert(0) += 1;
}

// `&self` does not promise "reads only": a `Cell` changes behind a shared borrow
struct Hits(Cell<u32>);

impl Hits {
    fn hit(&self) {
        self.0.set(self.0.get() + 1);
    }
}

fn call_fn(f: impl Fn()) {
    f();
    f();
}

fn main() {
    assert_eq!(same(5), 5);
    assert!(bigger(2.5, 1.0));
    let mut s = Store(HashMap::new());
    s.0.insert("k".to_string(), 7);
    assert_eq!(*s.get("k"), 7);
    let mut m = HashMap::new();
    bump(&mut m);
    bump(&mut m);
    assert_eq!(m["a"], 2);
    let h = Hits(Cell::new(0));
    h.hit();
    let cell = Cell::new(0);
    call_fn(|| cell.set(cell.get() + 1));
    assert_eq!((h.0.get(), cell.get()), (1, 2));
    // floats are PartialOrd but not Ord
    assert_eq!(2.0_f64.partial_cmp(&f64::NAN), None);
    // `fn` items and capture-less closures convert to `fn` pointers
    fn double(n: i32) -> i32 {
        n * 2
    }
    let p: fn(i32) -> i32 = double;
    let q: fn(i32) -> i32 = |x| x + 1;
    assert_eq!((p(2), q(2)), (4, 3));
    // a Copy type is copied by a by-value call, not moved
    let n = Some(4);
    let _ = n.map(|v| v + 1);
    assert_eq!(n, Some(4));
    println!("more checks out");
}
