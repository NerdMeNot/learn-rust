fn main() {
    let mut scores = vec![10, 20, 30];
    let first = &mut scores[0];
    let last = &mut scores[2];
    *first += 1;
    *last += 1;
    println!("{scores:?}");
}
