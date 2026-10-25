fn main() {
    let mut nums = vec![1, 2, 3];
    let view = &nums; // a shared borrow: nobody may change nums while it lives
    nums.push(4);
    println!("{}", view.len());
}
