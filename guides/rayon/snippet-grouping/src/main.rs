use rayon::prelude::*;

fn main() {
    // The same operator, grouped two ways, by hand.
    println!("(10 - 3) - 2 = {}", (10 - 3) - 2);
    println!("10 - (3 - 2) = {}", 10 - (3 - 2));
    println!("(0.1 + 0.2) + 0.3 = {}", (0.1 + 0.2) + 0.3);
    println!("0.1 + (0.2 + 0.3) = {}", 0.1 + (0.2 + 0.3));

    // `-` is not associative, so how rayon groups the items changes the answer.
    let numbers: Vec<i64> = (1..=1_000).collect();
    let sequential = numbers.iter().fold(0, |a, b| a - b);
    let parallel = numbers.par_iter().cloned().reduce(|| 0, |a, b| a - b);
    println!("sequential {sequential}, parallel {parallel}");
}
