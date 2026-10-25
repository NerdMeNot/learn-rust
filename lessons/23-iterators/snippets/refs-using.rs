fn main() {
    let nums = vec![3, 8, 12];
    let n: &i32 = &nums[1];

    // Arithmetic works through one &: Rust defines `&i32 * i32` and friends.
    println!("n * 2: {}", n * 2);
    println!("n % 3: {}", n % 3);

    // Method calls follow references automatically, however many there are.
    println!("n.pow(2): {}", n.pow(2));
    let nn: &&i32 = &n;
    println!("nn.is_positive(): {}", nn.is_positive());

    // Comparisons need the same type on both sides.
    println!("n == &8: {}", n == &8);
    println!("*n == 8: {}", *n == 8);

    // * follows a reference to the value. For an i32, that's a copy of the number.
    let value: i32 = *n;
    println!("*n: {value}");
    println!("**nn: {}", **nn);

    // A pattern in a closure's parameter does the same: |&&x| takes both & off.
    let big: Vec<&i32> = nums.iter().filter(|&&x| x > 5).collect();
    println!("filter(|&&x| x > 5): {big:?}");
}
