fn total(scores: Vec<u32>) -> u32 {
    scores.iter().sum()
}

fn main() {
    let scores = vec![90, 72, 85];
    let sum = total(scores);
    println!("sum: {sum}");
    println!("count: {}", scores.len());
}
