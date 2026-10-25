/// How many of the numbers in `stack` are prime (2, 3, 5, 7, 11, ...). 0 and 1 are not.
fn count_primes(mut stack: Vec<u32>) -> u32 {
    todo!("`while let Some(n) = stack.pop()`; a labelled `continue` when a divisor is found")
}

fn main() {
    assert_eq!(count_primes(vec![2, 3, 4, 5, 6, 7]), 4);
    assert_eq!(count_primes(vec![97, 100]), 1);
    assert_eq!(count_primes(vec![9, 15, 21]), 0);
    // 0 and 1 are not prime, and an empty stack is fine
    assert_eq!(count_primes(vec![0, 1]), 0);
    assert_eq!(count_primes(vec![]), 0);

    println!("all checks passed");
}
