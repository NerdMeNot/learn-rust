#[cxx::bridge]
mod ffi {
    // A struct declared here exists on both sides, with the same layout.
    struct Stats {
        count: usize,
        total: i32,
    }

    unsafe extern "C++" {
        include!("counter-cxx/src/counter.h");

        type Counter;

        fn new_counter(name: &str) -> UniquePtr<Counter>;
        fn add(self: Pin<&mut Counter>, value: i32);
        fn stats(self: &Counter) -> Stats;
        fn name(self: &Counter) -> &CxxString;
        // A C++ function that may throw: cxx turns the exception into an Err.
        fn at(self: &Counter, index: usize) -> Result<i32>;
        // This one calls back into Rust's `describe`.
        fn print_report(counter: &Counter);
    }

    extern "Rust" {
        // C++ code can call this one.
        fn describe(stats: &Stats) -> String;
    }
}

fn describe(stats: &ffi::Stats) -> String {
    format!("{} values, total {}", stats.count, stats.total)
}

fn main() {
    let mut counter = ffi::new_counter("clicks");
    counter.pin_mut().add(4);
    counter.pin_mut().add(5);

    let stats = counter.stats();
    println!("{}: {}", counter.name(), describe(&stats));

    ffi::print_report(&counter);

    println!("at(1) = {:?}", counter.at(1));
    match counter.at(9) {
        Ok(v) => println!("at(9) = {v}"),
        Err(e) => println!("at(9) failed: {e}"),
    }

    drop(counter);
    println!("Rust: after the drop");
}
