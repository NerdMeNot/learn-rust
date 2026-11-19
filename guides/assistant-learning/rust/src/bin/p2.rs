fn main() {
    let mut scores = vec![10, 20, 30];
    let first = &scores[0];
    println!("first: {first}");
    scores.push(40);
    println!("scores: {scores:?}");
}
