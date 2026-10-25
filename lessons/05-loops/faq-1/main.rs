fn main() {
    let nums = vec![3, 8, 12, 5];

    // only `loop` can hand back a value with break
    let mut i = 0;
    let idx = loop {
        if nums[i] > 10 { break i; }
        i += 1;
    };

    // what people usually write: iterator methods are expressions
    let first_big = nums.iter().find(|&&n| n > 10);
    let position = nums.iter().position(|&n| n > 10);
    let total: i32 = nums.iter().sum();
    let squares: Vec<i32> = nums.iter().map(|n| n * n).collect();
    println!("{idx} {first_big:?} {position:?} {total} {squares:?}");
}
