fn add_bonus(scores: &mut Vec<i32>) {
    scores.push(100);
}

fn main() {
    let mut a = vec![1, 2, 3];
    let mut b = a.clone();
    b.push(4);
    println!("{a:?} {b:?}");
    add_bonus(&mut a);
    println!("{a:?}");
}
