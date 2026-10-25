fn main() {
    let mut scores = vec![10, 20, 30];
    let best = &scores[2]; // a reference into the Vec
    scores.push(40); // pushing may move the Vec's buffer
    println!("best: {best}");
}
