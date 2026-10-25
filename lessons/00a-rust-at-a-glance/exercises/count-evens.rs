// Count how many numbers in the slice are even. Zero is even.
fn count_evens(nums: &[i32]) -> usize {
    todo!("`let mut count = 0;`, then `for n in nums` and `n % 2 == 0`")
}

fn main() {
    assert_eq!(count_evens(&[1, 2, 3, 4]), 2);
    assert_eq!(count_evens(&[]), 0); // an empty slice
    assert_eq!(count_evens(&[0]), 1); // zero is even
    assert_eq!(count_evens(&[-3, -2, -1]), 1); // negative numbers
    println!("all checks passed");
}
