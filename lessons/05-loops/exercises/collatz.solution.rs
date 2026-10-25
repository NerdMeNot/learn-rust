/// How many steps it takes to reach 1: an even n becomes n / 2, an odd n becomes 3 * n + 1.
/// `n` is at least 1. 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1 is 8 steps.
fn collatz_steps(mut n: u64) -> u32 {
    let mut steps = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    steps
}

/// The smallest k whose square is greater than `limit`.
fn first_square_over(limit: u32) -> u32 {
    let mut k = 1;
    loop {
        if k * k > limit {
            break k;
        }
        k += 1;
    }
}

fn main() {
    assert_eq!(collatz_steps(1), 0);
    assert_eq!(collatz_steps(2), 1);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(27), 111);

    assert_eq!(first_square_over(0), 1);
    assert_eq!(first_square_over(1), 2);
    assert_eq!(first_square_over(24), 5);
    assert_eq!(first_square_over(25), 6);
    assert_eq!(first_square_over(100), 11);

    println!("all checks passed");
}
