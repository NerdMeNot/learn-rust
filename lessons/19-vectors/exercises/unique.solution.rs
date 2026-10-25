// Return the items of `items` with repeats removed, keeping the first of each
// and the original order.
fn unique(items: &[i32]) -> Vec<i32> {
    let mut seen = Vec::new();
    for item in items {
        if !seen.contains(item) {
            seen.push(*item);
        }
    }
    seen
}

fn main() {
    assert_eq!(unique(&[3, 1, 3, 2, 1]), vec![3, 1, 2]);
    assert_eq!(unique(&[2, 1, 2]), vec![2, 1]); // repeats that are not next to each other
    assert_eq!(unique(&[7, 7, 7]), vec![7]);
    assert_eq!(unique(&[1, 2, 3]), vec![1, 2, 3]);
    assert_eq!(unique(&[]), Vec::<i32>::new());
    println!("all checks passed");
}
