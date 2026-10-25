fn main() {
    // reads like a pipeline: nothing is built between the steps, no temporary list
    let total: u64 = (1..=1_000_000u64)
        .filter(|n| n % 3 == 0)
        .map(|n| n * 2)
        .sum();
    println!("pipeline: {total}");

    // the same job as a loop written by hand
    let mut by_hand = 0u64;
    for n in 1..=1_000_000u64 {
        if n % 3 == 0 {
            by_hand += n * 2;
        }
    }
    println!("by hand:  {by_hand}");
    assert_eq!(total, by_hand);
}
