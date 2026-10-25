/// Returns (F(n), F(n + 1)), where F(0) = 0 and F(1) = 1.
fn fib_pair(n: u32) -> (u64, u64) {
    todo!("start from (0, 1); update both numbers at once, n times")
}

fn main() {
    assert_eq!(fib_pair(0), (0, 1));
    assert_eq!(fib_pair(1), (1, 1));
    assert_eq!(fib_pair(10), (55, 89));
    // F(93) is the largest Fibonacci number that fits in a u64.
    assert_eq!(fib_pair(92), (7540113804746346429, 12200160415121876738));

    // The caller unpacks the pair.
    let (this, next) = fib_pair(20);
    assert_eq!(next - this, 4181);

    println!("all checks passed");
}
