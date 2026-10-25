// The caller must pass a non-empty slice. An empty one is a bug in the caller,
// not bad input to recover from, so the function stops loudly.
fn average(scores: &[u32]) -> u32 {
    assert!(!scores.is_empty(), "average needs at least one score");
    scores.iter().sum::<u32>() / scores.len() as u32
}

fn main() {
    println!("average: {}", average(&[70, 80, 90]));
    println!("average: {}", average(&[]));
}
