fn main() {
    let nums = [3, 8, 12, 5, 20];

    println!("any > 10: {}", nums.iter().any(|&n| n > 10));
    println!("all > 0: {}", nums.iter().all(|&n| n > 0));
    // On an empty iterator, any is false and all is true.
    let empty: [i32; 0] = [];
    println!(
        "empty any: {}, empty all: {}",
        empty.iter().any(|&n| n > 0),
        empty.iter().all(|&n| n > 0)
    );

    // find returns the first match; position returns its index.
    println!("find > 10: {:?}", nums.iter().find(|&&n| n > 10));
    println!("position > 10: {:?}", nums.iter().position(|&n| n > 10));
    println!("find > 100: {:?}", nums.iter().find(|&&n| n > 100));
    println!("rposition > 10: {:?}", nums.iter().rposition(|&n| n > 10));

    // find_map: the first item for which the closure returns Some.
    let lines = ["# comment", "port=8080", "host=x"];
    let port = lines.iter().find_map(|l| l.strip_prefix("port="));
    println!("find_map port: {port:?}");

    // contains is on slices, not iterators.
    println!("contains(&12): {}", nums.contains(&12));
}
