fn main() {
    // Each total needs the one before it, so an index is the natural tool.
    let mut totals = vec![3, 1, 4, 1, 5];
    for i in 1..totals.len() {
        totals[i] += totals[i - 1];
    }
    println!("running totals: {totals:?}");

    // On an empty list, 1..0 is an empty range: nothing runs, nothing panics.
    let mut empty: Vec<i32> = vec![];
    for i in 1..empty.len() {
        empty[i] += empty[i - 1];
    }
    println!("empty: {empty:?}");
}
