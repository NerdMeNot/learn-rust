use rayon::prelude::*;
use std::thread;

fn thread_name() -> String {
    thread::current().name().unwrap_or("?").to_string()
}

fn main() {
    // Defaults: each pool sizes itself from the core count, and neither asks the other.
    let cores = thread::available_parallelism().unwrap().get();
    let default_rt = tokio::runtime::Runtime::new().unwrap();
    assert_eq!(default_rt.metrics().num_workers(), cores);
    assert_eq!(rayon::current_num_threads(), cores);
    println!("by default, Tokio workers and rayon threads each match available_parallelism");
    drop(default_rt);

    // Set both by hand: two numbers you chose, two sets of threads.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("io")
        .build()
        .unwrap();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(3)
        .thread_name(|i| format!("cpu-{i}"))
        .build()
        .unwrap();
    println!("Tokio workers: {}", rt.metrics().num_workers());
    println!("rayon threads: {}", pool.current_num_threads());

    // Where does code run? An async task runs on a Tokio worker; par_iter inside
    // install() runs on the rayon pool's threads.
    let task = rt.block_on(async { tokio::spawn(async { thread_name() }).await.unwrap() });
    let mut rayon_names: Vec<String> =
        pool.install(|| (0..100).into_par_iter().map(|_| thread_name()).collect());
    rayon_names.sort();
    rayon_names.dedup();
    println!("async task ran on a thread named {task:?}");
    println!(
        "par_iter ran only on cpu-* threads: {}",
        rayon_names.iter().all(|n| n.starts_with("cpu-"))
    );
    assert_eq!(task, "io");
    assert!(rayon_names.iter().all(|n| n.starts_with("cpu-")) && rayon_names.len() <= 3);
}
