#![deny(unused_comparisons)] // this lint is a warning by default; deny makes it an error

fn main() {
    let items = [5, 6, 7];
    let mut i = items.len() - 1;
    while i >= 0 {
        println!("items[{i}] = {}", items[i]);
        i -= 1;
    }
}
