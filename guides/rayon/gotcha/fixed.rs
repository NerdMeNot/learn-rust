use rayon::prelude::*;

fn main() {
    let total: u64 = (1..=1_000_000u64).into_par_iter().sum();
    println!("{total}");
}
