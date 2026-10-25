fn main() {
    // a worklist: each pass may add more work, which `for x in &work` would not allow
    let mut work = vec![1];
    let mut order = Vec::new();
    while let Some(n) = work.pop() {
        order.push(n);
        if n * 2 <= 7 {
            work.push(n * 2);
            work.push(n * 2 + 1);
        }
    }
    println!("visited: {order:?}");
    println!("work left: {work:?}");
}
