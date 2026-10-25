fn main() {
    let mut n = 0;
    let first = &mut n;
    let second = first; // moves the `&mut`, like moving a String
    *first += 1;
    *second += 1;
    println!("n: {n}");
}
