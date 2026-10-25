fn main() {
    let mut n = 1;
    let a = &raw mut n;
    let b = a; // a second pointer to the same place: the compiler tracks neither
    // SAFETY: `a` and `b` are copies of one pointer to `n`, which is alive, and the
    // two writes happen one after the other
    unsafe {
        *a += 10;
        *b += 100;
    }
    println!("n = {n}");
}
