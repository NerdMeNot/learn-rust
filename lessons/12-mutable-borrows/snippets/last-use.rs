fn main() {
    let mut scores = vec![10, 20];
    let writer = &mut scores;
    writer.push(30); // the last use of `writer`: its borrow ends here
    println!("scores: {scores:?}");

    scores.push(40); // this borrows `scores` mutably for the call
    println!("scores: {scores:?}");
}
