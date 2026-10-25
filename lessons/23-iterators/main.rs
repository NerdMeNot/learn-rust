fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    let evens_squared: Vec<i32> = nums
        .iter()                     // items are &i32 (borrowed from nums)
        .filter(|&&n| n % 2 == 0)   // filter lends each item again: &&i32
        .map(|n| n * n)             // map gets the item itself: &i32
        .collect();                 // build a Vec from the results

    let total: i32 = nums.iter().sum();
    println!("{evens_squared:?} {total}");
}
