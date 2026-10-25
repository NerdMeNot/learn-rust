// Fix 1: the function only reads, so its signature should borrow.
fn total(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

// This one keeps what it is given, so it must own it.
struct Report {
    scores: Vec<u32>,
}

impl Report {
    fn new(scores: Vec<u32>) -> Report {
        Report { scores }
    }
}

fn main() {
    let scores = vec![90, 72, 85];
    println!("total: {}", total(&scores));

    // Fix 2: clone, when the caller needs its own copy too.
    let report = Report::new(scores.clone());
    println!(
        "clone, same buffer: {}",
        report.scores.as_ptr() == scores.as_ptr()
    );

    // Fix 3: give it away, and stop using the original.
    let before = scores.as_ptr();
    let report = Report::new(scores);
    println!("move, same buffer: {}", report.scores.as_ptr() == before);
}
