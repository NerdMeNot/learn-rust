fn add_bonus(scores: &mut Vec<i32>, bonus: i32) {
    // iter_mut() visits each number as a &mut, a reference that can change it
    for s in scores.iter_mut() {
        *s += bonus; // *s is the number that s points at
    }
    scores.push(0);
}

fn main() {
    let mut scores = vec![10, 20]; // vec![...] makes a growable list
    add_bonus(&mut scores, 5);
    println!("{scores:?}");
}
