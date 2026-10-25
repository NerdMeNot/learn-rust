fn main() {
    let x = 4;
    // in Rust `&` binds tighter than `==`, so this reads (x & 1) == 0
    if x & 1 == 0 {
        println!("{x} is even");
    } else {
        println!("{x} is odd");
    }
}
