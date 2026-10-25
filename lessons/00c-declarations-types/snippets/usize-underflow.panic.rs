fn last_index(items: &[i32]) -> usize {
    items.len() - 1 // `len()` is a usize: a whole number that cannot go below 0
}

fn main() {
    println!("last_index: {}", last_index(&[1, 2, 3]));
    println!("last_index: {}", last_index(&[]));
}
