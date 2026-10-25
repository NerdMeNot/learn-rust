fn main() {
    let mut scores = vec![10, 20, 30];
    scores[0] += 1; // simplest: one change at a time
    scores[2] += 1;

    // two at once: split into halves that can't overlap
    let (left, right) = scores.split_at_mut(2);
    left[0] += 1;
    right[0] += 1;
    println!("{scores:?}");
}
