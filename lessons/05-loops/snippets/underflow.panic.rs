#![allow(unused_comparisons)] // silences the compiler's "this is always true" warning

fn main() {
    let items = [5, 6, 7];
    let mut i = items.len() - 1;
    // a usize is never below 0, so `i >= 0` is always true
    while i >= 0 {
        println!("items[{i}] = {}", items[i]);
        i -= 1;
    }
}
