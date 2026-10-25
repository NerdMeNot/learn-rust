fn main() {
    let nums = vec![3, 8, 12, 5];
    let found = for n in &nums {
        if *n > 10 { break *n; }
    };
}
