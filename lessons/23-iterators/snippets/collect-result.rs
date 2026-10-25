fn main() {
    let good = ["1", "2", "3"];
    let bad = ["1", "two", "3", "four"];

    // Each parse gives a Result. Collecting into Vec<Result<..>> keeps them all.
    let each: Vec<Result<i32, _>> = bad.iter().map(|s| s.parse::<i32>()).collect();
    println!("Vec<Result>: {each:?}");

    // Collecting into Result<Vec<..>, ..> flips it: Ok with every value,
    // or the first Err.
    let all: Result<Vec<i32>, _> = good.iter().map(|s| s.parse::<i32>()).collect();
    println!("good -> {all:?}");

    // It stops at the first Err. inspect shows "3" and "four" are never parsed.
    let all: Result<Vec<i32>, _> = bad
        .iter()
        .inspect(|s| println!("  parsing {s:?}"))
        .map(|s| s.parse::<i32>())
        .collect();
    println!("bad  -> {all:?}");

    // Option works the same way: Some(Vec) only if every item is Some.
    let firsts: Option<Vec<char>> = ["ab", "cd"].iter().map(|s| s.chars().next()).collect();
    println!("all Some -> {firsts:?}");
    let firsts: Option<Vec<char>> = ["ab", ""].iter().map(|s| s.chars().next()).collect();
    println!("one None -> {firsts:?}");
}
