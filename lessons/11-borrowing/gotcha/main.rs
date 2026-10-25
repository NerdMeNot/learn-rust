fn total(xs: Vec<i32>) -> i32 {
    let mut sum = 0;
    for x in xs {
        sum += x;
    }
    sum
}

fn main() {
    let nums = vec![1, 2, 3];
    let t = total(nums);
    println!("{t} {nums:?}");
}
