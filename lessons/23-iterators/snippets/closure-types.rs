fn main() {
    let nums = vec![1, 2, 3, 4];

    // Parameter types are written out here, so the compiler checks every claim.

    // .iter() hands out &i32, and map gets each item as it is.
    let a: Vec<i32> = nums.iter().map(|n: &i32| n * 10).collect();
    // filter only looks, so it borrows the item: a & on top of &i32.
    let b: Vec<&i32> = nums.iter().filter(|n: &&i32| **n > 2).collect();
    println!("iter(): map {a:?}, filter {b:?}");

    // A pattern in the parameter peels the & off for you.
    let c: Vec<&i32> = nums.iter().filter(|&&n| n > 2).collect();
    println!("|&&n| pattern: {c:?}");

    // copied() turns &i32 into i32 first. Then there is one & less everywhere.
    let d: Vec<i32> = nums.iter().copied().filter(|n: &i32| *n > 2).collect();
    let e: Vec<i32> = nums.iter().copied().filter(|&n| n > 2).collect();
    println!("copied(): {d:?} {e:?}");

    // For types that aren't Copy, cloned() makes owned copies.
    let names = vec![String::from("ana"), String::from("bo")];
    let owned: Vec<String> = names.iter().filter(|s| s.len() > 2).cloned().collect();
    println!("cloned(): {owned:?}, names still here: {names:?}");

    // into_iter() on the Vec itself hands out the items, so there is nothing
    // to copy, but `names` is used up.
    let shouted: Vec<String> = names
        .into_iter()
        .map(|s: String| s.to_uppercase())
        .collect();
    println!("into_iter(): {shouted:?}");
}
