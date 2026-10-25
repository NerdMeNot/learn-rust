fn main() {
    let nums = [4, 9, 2, 9, 1];
    // Option, because an empty iterator has no max.
    println!("max: {:?}, min: {:?}", nums.iter().max(), nums.iter().min());
    let empty: [i32; 0] = [];
    println!("max of empty: {:?}", empty.iter().max());

    // _by_key: compare by something computed from each item.
    let words = ["kiwi", "fig", "banana", "plum"];
    println!("longest: {:?}", words.iter().max_by_key(|w| w.len()));
    println!("shortest: {:?}", words.iter().min_by_key(|w| w.len()));
    // On a tie, max_ returns the last of the equals and min_ the first.
    println!(
        "max_by_key tie (kiwi, plum): {:?}",
        ["kiwi", "plum"].iter().max_by_key(|w| w.len())
    );

    // f64 is not Ord (NaN isn't bigger or smaller than anything),
    // so max() won't compile. total_cmp gives floats a full order.
    let temps: [f64; 3] = [21.5, 19.0, 23.25];
    let hottest = temps.iter().copied().max_by(|a, b| a.total_cmp(b));
    println!("hottest: {hottest:?}");

    // Or fold with f64::min (or f64::max), which ignore NaN.
    let coldest = temps.iter().copied().fold(f64::INFINITY, f64::min);
    println!("coldest: {coldest}");

    // Floats by key: compare with total_cmp inside max_by.
    let cities: [(&str, f64); 3] = [("Oslo", 4.5), ("Lima", 19.0), ("Rome", 15.5)];
    let warmest = cities.iter().max_by(|a, b| a.1.total_cmp(&b.1));
    println!("warmest city: {warmest:?}");
}
