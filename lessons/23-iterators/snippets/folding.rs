fn main() {
    let nums = [1, 2, 3, 4, 5, 6];

    // fold: start with a value and combine every item into it.
    let sum_of_squares = nums.iter().fold(0, |acc, n| acc + n * n);
    println!("fold sum of squares: {sum_of_squares}");
    let csv = nums.iter().fold(String::new(), |mut acc, n| {
        if !acc.is_empty() {
            acc.push(',');
        }
        acc.push_str(&n.to_string());
        acc
    });
    println!("fold into a String: {csv}");

    // reduce: like fold, but the first item is the start. None if empty.
    println!("reduce: {:?}", nums.iter().copied().reduce(|a, b| a * b));

    // partition: split into two collections by a test.
    let (even, odd): (Vec<i32>, Vec<i32>) = nums.iter().partition(|&&n| n % 2 == 0);
    println!("partition: even {even:?}, odd {odd:?}");

    // unzip: a list of pairs into a pair of lists.
    let pairs = [("ana", 28), ("bo", 31)];
    let (names, ages): (Vec<&str>, Vec<i32>) = pairs.iter().copied().unzip();
    println!("unzip: {names:?} {ages:?}");

    // for_each: run a closure on every item. Same as a for loop;
    // handy at the end of a long chain.
    nums.iter()
        .filter(|&&n| n > 4)
        .for_each(|n| println!("for_each: {n}"));
}
