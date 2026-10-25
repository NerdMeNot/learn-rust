fn main() {
    let mut names = vec!["file10", "file2", "Zoë", "éclair", "zebra", "File1"];

    // sort compares bytes: capitals first, é after z, "file10" before "file2".
    names.sort();
    println!("sort:                 {names:?}");

    // A lowercase key fixes the capitals, and nothing else.
    // sort_by_cached_key calls the key function once per element.
    names.sort_by_cached_key(|n| n.to_lowercase());
    println!("sort_by_cached_key:   {names:?}");

    // The comparison itself, as bytes.
    println!("\"é\" > \"z\":   {}", "é" > "z");
    println!("\"é\" bytes:   {:?}", "é".as_bytes());
    println!("\"z\" bytes:   {:?}", "z".as_bytes());
}
