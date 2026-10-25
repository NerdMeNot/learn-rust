// Backs the table cells that show a Rust expression and its result.
use std::sync::Mutex;

const fn square(n: u32) -> u32 {
    n * n
}

static LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
const AREA: u32 = square(4);
const NAMES: [&str; 2] = ["a", "b"];
const ORIGIN: (i32, i32) = (0, 0);

fn main() {
    // `let x = x + 1` reads the OLD x, because the right side runs before the new name exists
    let x = 5;
    let x = x + 1;
    assert_eq!(x, 6);
    // a value chosen by a condition, set once
    let c = true;
    let a = if c { 1 } else { 2 };
    let b;
    if c {
        b = 1;
    } else {
        b = 2;
    }
    assert_eq!((a, b), (1, 1));
    // a const fn runs at compile time in a const and at run time in a call
    let n = std::env::args().count() as u32;
    assert_eq!(AREA, 16);
    assert_eq!(square(n + 2), 9);
    // a Mutex in a static changes without `mut` or `unsafe`
    LOG.lock().unwrap().push(7);
    assert_eq!(*LOG.lock().unwrap(), vec![7]);
    // a const is not limited to numbers and text
    assert_eq!((NAMES.len(), ORIGIN), (2, (0, 0)));
}
