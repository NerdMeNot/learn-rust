fn main() {
    let threads = rayon::current_num_threads();
    match std::env::var("RAYON_NUM_THREADS") {
        Ok(v) => println!("RAYON_NUM_THREADS={v}, the pool has {threads} threads"),
        Err(_) => {
            let cpus = std::thread::available_parallelism().unwrap().get();
            println!("the pool has at least one thread: {}", threads >= 1);
            println!("the pool has one thread per CPU: {}", threads == cpus);
        }
    }
}
