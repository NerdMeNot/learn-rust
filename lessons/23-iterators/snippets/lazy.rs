fn main() {
    let nums = vec![1, 2, 3];

    // Build a chain. The closure prints, so we can see when it runs.
    let doubled = nums.iter().map(|n| {
        println!("  map sees {n}");
        n * 2
    });
    println!("chain built, nothing has run yet");

    // collect pulls every item through the chain.
    let result: Vec<i32> = doubled.collect();
    println!("collected: {result:?}");

    // Under the hood, everything is calls to next().
    let mut it = nums.iter();
    println!("next: {:?}", it.next());
    println!("next: {:?}", it.next());
    println!("next: {:?}", it.next());
    println!("next: {:?}", it.next()); // used up: None from now on
}
