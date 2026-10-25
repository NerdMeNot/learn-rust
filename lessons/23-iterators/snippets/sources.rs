use std::collections::BTreeMap;
use std::iter;

fn main() {
    // Ranges are iterators already.
    let up: Vec<i32> = (1..5).collect();
    let up_to: Vec<i32> = (1..=5).collect();
    println!("1..5: {up:?}   1..=5: {up_to:?}");

    // Text: see the strings lesson for more.
    let text = "one two\nthree";
    println!("chars: {:?}", "héllo".chars().collect::<Vec<_>>());
    println!("lines: {:?}", text.lines().collect::<Vec<_>>());
    println!("split: {:?}", "a,b,,c".split(',').collect::<Vec<_>>());

    // Maps give (key, value) pairs. BTreeMap keeps keys sorted;
    // a HashMap gives them in an order that changes from run to run.
    let ages = BTreeMap::from([("bo", 31), ("ana", 28)]);
    for (name, age) in &ages {
        println!("map: {name} is {age}");
    }
    println!("keys: {:?}", ages.keys().collect::<Vec<_>>());
    println!("values: {:?}", ages.values().collect::<Vec<_>>());

    // Making iterators from nothing.
    let dashes: String = iter::repeat('-').take(5).collect();
    println!("repeat: {dashes}");
    println!(
        "repeat_n: {:?}",
        iter::repeat_n("hi", 3).collect::<Vec<_>>()
    );
    println!("once: {:?}", iter::once(42).collect::<Vec<_>>());

    // successors: each item is computed from the previous one.
    // checked_mul returns None on overflow, which ends the sequence.
    let powers: Vec<u16> = iter::successors(Some(1u16), |&n| n.checked_mul(10)).collect();
    println!("successors: {powers:?}");

    // from_fn: a closure that returns Some(item) or None to stop.
    let mut count = 0;
    let counter = iter::from_fn(|| {
        count += 1;
        if count <= 3 { Some(count) } else { None }
    });
    println!("from_fn: {:?}", counter.collect::<Vec<_>>());
}
