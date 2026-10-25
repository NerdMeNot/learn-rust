fn main() {
    let mut nums = vec![30, 5, 12, 5, 99];
    nums.sort();
    println!("sort:              {nums:?}");
    nums.sort_by(|a, b| b.cmp(a));
    println!("sort_by (reverse): {nums:?}");

    let mut words = vec!["pear", "fig", "banana", "kiwi"];
    words.sort_by_key(|w| w.len());
    println!("sort_by_key(len):  {words:?}");

    let mut ranked = vec![("ana", 90), ("bo", 75), ("cy", 90), ("di", 75)];
    ranked.sort_by_key(|p| p.1);
    println!("sort_by_key(score): {ranked:?}");

    let mut quick = vec![3, 1, 2];
    quick.sort_unstable();
    println!("sort_unstable:     {quick:?}");

    let sorted = vec![5, 5, 12, 30, 99];
    println!("contains(&12):     {}", sorted.contains(&12));
    println!("binary_search(&12): {:?}", sorted.binary_search(&12));
    println!("binary_search(&13): {:?}", sorted.binary_search(&13));
    println!("binary_search(&0):  {:?}", sorted.binary_search(&0));
}
