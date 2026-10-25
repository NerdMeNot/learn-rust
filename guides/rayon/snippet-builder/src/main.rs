use rayon::prelude::*;

fn main() {
    // Configure the global pool once, before anything uses it.
    let first = rayon::ThreadPoolBuilder::new().num_threads(2).build_global();
    println!("first build_global: {first:?}");
    println!("global pool threads: {}", rayon::current_num_threads());
    match rayon::ThreadPoolBuilder::new().num_threads(4).build_global() {
        Ok(()) => println!("second build_global worked"),
        Err(e) => println!("second build_global: {e}"),
    }

    // A pool of your own: `install` runs the closure inside it.
    let pool = rayon::ThreadPoolBuilder::new().num_threads(3).build().unwrap();
    let inside = pool.install(|| rayon::current_num_threads());
    let total: u64 = pool.install(|| (1..=1_000u64).into_par_iter().sum());
    println!("inside install: {inside} threads, sum {total}");
    println!("outside install: still {} threads", rayon::current_num_threads());
}
