/// How many numbers from 1 to `limit` (inclusive) are multiples of `step`?
/// A `step` of 0 has no multiples, so the answer is 0.
/// This version compiles but gives the wrong answer. Read the compiler's warnings: they point at the bug.
fn count_multiples(limit: u32, step: u32) -> u32 {
    if step == 0 {
        return 0;
    }
    let mut count = 0;
    for n in 1..=limit {
        if n % step == 0 {
            let count = count + 1;
        }
    }
    count
}

fn main() {
    assert_eq!(count_multiples(10, 3), 3); // 3, 6, 9
    assert_eq!(count_multiples(10, 1), 10);
    assert_eq!(count_multiples(2, 5), 0);
    assert_eq!(count_multiples(0, 3), 0);
    // The case to watch: a step of 0 must not crash.
    assert_eq!(count_multiples(10, 0), 0);
    println!("all checks passed");
}
