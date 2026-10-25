// The sum of every run of `size` neighbouring numbers, left to right.
// If `size` is 0 or longer than the slice, there are no runs: return an empty Vec.
fn window_sums(xs: &[i32], size: usize) -> Vec<i32> {
    todo!("`xs.windows(size)` gives each run as a slice, but it panics when size is 0")
}

fn main() {
    assert_eq!(window_sums(&[1, 2, 3, 4, 5], 3), vec![6, 9, 12]);
    assert_eq!(window_sums(&[4, 5], 1), vec![4, 5]);
    assert_eq!(window_sums(&[1, 2, 3], 3), vec![6]);
    assert_eq!(window_sums(&[1, 2], 3), Vec::<i32>::new());
    assert_eq!(window_sums(&[], 2), Vec::<i32>::new());
    assert_eq!(window_sums(&[5, 6], 0), Vec::<i32>::new());
    println!("all checks passed");
}
