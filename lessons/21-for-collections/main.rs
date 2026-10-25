fn main() {
    let names = vec![String::from("ana"), String::from("bo")];

    // Borrow each item: names stays usable.
    for n in &names {
        println!("hi {n}");
    }

    let mut nums = vec![1, 2, 3];
    // Borrow each item so it can be changed.
    for n in &mut nums {
        *n *= 10;
    }

    // Each item with its position.
    for (i, n) in nums.iter().enumerate() {
        println!("{i}: {n}");
    }
    println!("{} names, {nums:?}", names.len());
}
