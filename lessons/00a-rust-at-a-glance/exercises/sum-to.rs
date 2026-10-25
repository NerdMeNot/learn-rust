// Return 1 + 2 + ... + n, counting n itself. For n = 0 the answer is 0.
fn sum_to(n: u32) -> u32 {
    todo!("start with `let mut total = 0;`, then `for i in 1..=n` and `total += i`")
}

fn main() {
    assert_eq!(sum_to(4), 10);
    assert_eq!(sum_to(1), 1); // `1..n` would stop one short and give 0
    assert_eq!(sum_to(0), 0); // `1..=0` has nothing in it
    assert_eq!(sum_to(100), 5050);
    println!("all checks passed");
}
