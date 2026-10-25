fn main() {
    let nums = vec![1, 2, 3];
    let evens = nums.iter().filter(|&&n| n % 2 == 0);
    let how_many = evens.count();
    let total: i32 = evens.sum();
    println!("{how_many} {total}");
}
