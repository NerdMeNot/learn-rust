fn largest(scores: &mut [i32]) -> &mut i32 {
    let mut best = 0;
    for i in 1..scores.len() {
        if scores[i] > scores[best] {
            best = i;
        }
    }
    &mut scores[best]
}

fn main() {
    let mut scores = vec![3, 9, 4];
    *largest(&mut scores) += 100;
    println!("scores: {scores:?}");
}
