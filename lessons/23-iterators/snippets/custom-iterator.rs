// Fibonacci numbers, one at a time. The struct holds the state between calls.
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64; // what next() hands out

    // The only method you have to write.
    fn next(&mut self) -> Option<u64> {
        let value = self.current;
        // checked_add returns None on overflow, which ends the sequence.
        let after = self.current.checked_add(self.next)?;
        self.current = self.next;
        self.next = after;
        Some(value)
    }
}

fn main() {
    // Every adaptor and consumer now works, without writing any of them.
    let first: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("take(10): {first:?}");

    let even_sum: u64 = Fibonacci::new()
        .take_while(|&n| n < 4_000_000)
        .filter(|n| n % 2 == 0)
        .sum();
    println!("sum of even ones under 4 million: {even_sum}");

    let first_big = Fibonacci::new().position(|n| n > 1000);
    println!("position of first > 1000: {first_big:?}");

    for (i, n) in Fibonacci::new().enumerate().skip(20).step_by(10) {
        println!("fib({i}) = {n}");
    }

    // It stops by itself before u64 overflows.
    println!(
        "count (it stops before u64 overflows): {}",
        Fibonacci::new().count()
    );
}
