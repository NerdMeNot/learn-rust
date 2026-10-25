/// How many steps until `start` reaches 1? One step: if the number is even, halve it;
/// if it is odd, triple it and add one. `start` is at least 1.
fn collatz_steps(start: u64) -> u32 {
    todo!("a `loop` that ends with `break steps`; the next number is an `if` expression")
}

fn main() {
    assert_eq!(collatz_steps(1), 0); // already there: zero steps
    assert_eq!(collatz_steps(2), 1);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(7), 16);
    assert_eq!(collatz_steps(27), 111);
    println!("all checks passed");
}
