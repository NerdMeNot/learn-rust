use std::thread;

fn parse(text: &'static str) -> thread::JoinHandle<Result<u32, std::num::ParseIntError>> {
    thread::spawn(move || text.parse::<u32>())
}

fn main() {
    // two layers: join says whether the thread panicked, the inner Result is the job's own
    let good = parse("17").join();
    let bad = parse("seventeen").join();
    println!("good: {:?}", good);
    println!("bad: {:?}", bad);

    // flatten both layers with ? in a function that returns a Result
    fn total() -> Result<u32, Box<dyn std::error::Error>> {
        let a = parse("20").join().map_err(|_| "thread panicked")??;
        let b = parse("22").join().map_err(|_| "thread panicked")??;
        Ok(a + b)
    }
    println!("total: {:?}", total().unwrap());
}
