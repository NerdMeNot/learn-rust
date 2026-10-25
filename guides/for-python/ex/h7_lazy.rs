fn square(n: i32) -> i32 {
    println!("squaring {n}");
    n * n
}

fn main() {
    let nums = [1, 2, 3];
    nums.iter().map(|&n| square(n));
    println!("chain written, never consumed");
    let lazy = nums.iter().map(|&n| square(n));
    println!("iterator made");
    let list: Vec<i32> = lazy.collect();
    println!("{list:?}");
}
