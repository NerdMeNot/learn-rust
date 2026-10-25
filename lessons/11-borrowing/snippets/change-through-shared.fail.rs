fn add_bonus(scores: &Vec<i32>) {
    scores.push(100);
}

fn main() {
    let scores = vec![30, 95, 60];
    add_bonus(&scores);
    println!("{scores:?}");
}
