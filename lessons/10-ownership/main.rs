fn main() {
    let a = vec![1, 2, 3];
    let b = a;              // the Vec MOVES to b. a can no longer be used.
    println!("{b:?}");

    let x = 5;
    let y = x;              // small values like numbers are copied instead
    println!("{x} {y}");
}
