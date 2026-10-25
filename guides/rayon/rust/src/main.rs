use rayon::prelude::*;

fn main() {
    let numbers: Vec<u64> = (1..=1_000_000).collect();

    // The same pipeline twice: only `iter` became `par_iter`.
    let sequential: u64 = numbers.iter().map(|n| n * n).sum();
    let parallel: u64 = numbers.par_iter().map(|n| n * n).sum();
    println!("sum of squares: {sequential} and {parallel}");
    assert_eq!(sequential, parallel);

    // `collect` into a Vec keeps the input order, even for a million items.
    let squares: Vec<u64> = numbers.par_iter().map(|n| n * n).collect();
    let expected: Vec<u64> = numbers.iter().map(|n| n * n).collect();
    println!("collected squares in input order: {}", squares == expected);

    // `filter` then `count`.
    let sevens = numbers.par_iter().filter(|n| **n % 7 == 0).count();
    println!("multiples of 7: {sevens}");

    // `par_iter_mut` changes the items in place.
    let mut data = vec![3, 1, 2];
    data.par_iter_mut().for_each(|n| *n *= 10);
    println!("times ten, in place: {data:?}");

    // `par_sort` sorts a slice or Vec using several threads.
    data.par_sort();
    println!("sorted: {data:?}");
}
