use std::collections::BTreeSet;

// Accepts anything a `for` loop accepts, as long as it yields i32.
fn total(items: impl IntoIterator<Item = i32>) -> i32 {
    let mut sum = 0;
    for n in items {
        sum += n;
    }
    sum
}

fn main() {
    println!("Vec: {}", total(vec![1, 2, 3]));
    println!("array: {}", total([4, 5]));
    println!("range: {}", total(1..=10));
    println!("set: {}", total(BTreeSet::from([7, 7, 8])));
    println!("chain: {}", total((1..=3).map(|n| n * n)));

    // `for x in v` is `IntoIterator::into_iter(v)` plus a loop over next().
    // Written out by hand:
    let v = vec!["a", "b"];
    let mut it = IntoIterator::into_iter(v);
    while let Some(x) = it.next() {
        println!("by hand: {x}");
    }

    // &Vec and &mut Vec implement IntoIterator too. That is why
    // `for x in &v` borrows and `for x in &mut v` lets you change items.
    let mut w = vec![1, 2];
    for x in &mut w {
        *x *= 100;
    }
    for x in &w {
        println!("after &mut loop: {x}");
    }
}
