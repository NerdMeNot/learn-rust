fn add_bonus(scores: &mut Vec<u32>) {
    scores.push(10);
}

fn add_penalty(scores: &mut Vec<u32>) {
    scores.push(0);
}

fn main() {
    // One owner. Each party borrows the list for the length of one call.
    let mut scores = vec![1, 2, 3];
    add_bonus(&mut scores);
    add_penalty(&mut scores);
    println!("scores: {scores:?}");

    // A second list of its own, when the two must not see each other's changes.
    let mut copy = scores.clone();
    copy.push(99);
    println!("scores: {scores:?}, copy: {copy:?}");
}
