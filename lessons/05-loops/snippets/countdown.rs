fn show(items: &[i32]) {
    // (0..len).rev() counts len-1, ..., 0 and never subtracts below zero
    for i in (0..items.len()).rev() {
        println!("items[{i}] = {}", items[i]);
    }
    println!("done, {} items", items.len());
}

fn main() {
    show(&[5, 6, 7]);
    show(&[]);
}
