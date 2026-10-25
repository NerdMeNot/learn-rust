fn main() {
    let mut scores = vec![3, 9, 4];
    let top = &scores[..2];
    scores.push(7);
    println!("top: {top:?}");
}
