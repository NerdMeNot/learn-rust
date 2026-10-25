use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // measuring: the exact value changes every run, so only a bound is printed
    let start = Instant::now();
    thread::sleep(Duration::from_millis(20));
    let elapsed = start.elapsed();
    println!("elapsed >= 20ms: {}", elapsed >= Duration::from_millis(20));

    let first = Instant::now();
    let second = Instant::now();
    println!("second >= first: {}", second >= first);

    // an Instant built by adding a Duration, so these answers do not depend on the clock
    let later = start + Duration::from_secs(1);
    println!("later - start: {:?}", later.duration_since(start));
    println!("start - later: {:?}", start.duration_since(later));
    println!(
        "saturating, wrong order: {:?}",
        start.saturating_duration_since(later)
    );
    println!(
        "checked, right order: {:?}",
        later.checked_duration_since(start)
    );
    println!(
        "checked, wrong order: {:?}",
        start.checked_duration_since(later)
    );
}
