fn total(xs: &[i32]) -> i32 {      // & means: borrow, don't take
    let mut sum = 0;
    for x in xs {
        sum += x;
    }
    sum
}

fn main() {
    let nums = vec![1, 2, 3];
    let t = total(&nums);       // lend nums to total
    println!("{t} {nums:?}");   // nums is still ours
}
