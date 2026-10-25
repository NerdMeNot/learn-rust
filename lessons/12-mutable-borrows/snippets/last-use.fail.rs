fn main() {
    let mut scores = vec![10, 20];
    let writer = &mut scores;
    println!("scores: {scores:?}");
    writer.push(30); // `writer` is used again, so its borrow was still alive above
}
