fn main() {
    let mut queue = vec![1, 2];
    for n in &queue {
        if *n < 20 {
            queue.push(n * 3);
        }
    }
    println!("{queue:?}");
}
