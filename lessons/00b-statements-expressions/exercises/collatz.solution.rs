/// How many steps until `start` reaches 1? One step: if the number is even, halve it;
/// if it is odd, triple it and add one. `start` is at least 1.
fn collatz_steps(start: u64) -> u32 {
    let mut n = start;
    let mut steps = 0;
    loop {
        if n == 1 {
            break steps;
        }
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
}

fn main() {
    assert_eq!(collatz_steps(1), 0); // already there: zero steps
    assert_eq!(collatz_steps(2), 1);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(7), 16);
    assert_eq!(collatz_steps(27), 111);
    println!("all checks passed");
}
