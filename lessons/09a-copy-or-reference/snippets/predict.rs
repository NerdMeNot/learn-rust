fn main() {
    let mut scores = vec![10, 20, 30];
    let mut copy = scores.clone();
    for s in &mut copy {
        *s += 1;
    }
    let first = scores[0];
    scores[0] = 99;
    let total: i32 = scores.iter().sum();
    println!("scores = {scores:?}");
    println!("copy   = {copy:?}");
    println!("first  = {first}");
    println!("total  = {total}");
}
