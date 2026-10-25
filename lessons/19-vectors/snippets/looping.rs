fn main() {
    let mut scores = vec![70, 85, 90];

    for s in &scores {
        println!("read: {s}");
    }

    for s in &mut scores {
        *s += 5;
    }
    println!("after +5: {scores:?}");

    for (i, s) in scores.iter().enumerate() {
        println!("score {i} is {s}");
    }

    let mut total = 0;
    for s in scores {
        total += s;
    }
    println!("total: {total}");
    // `scores` was moved into the loop, so it cannot be used here.
}
