// The sum of every run of `size` neighbouring numbers, left to right.
// If `size` is 0 or longer than the slice, there are no runs: return an empty Vec.
fn window_sums(xs: &[i32], size: usize) -> Vec<i32> {
    let mut sums = Vec::new();
    if size == 0 {
        return sums;
    }
    for run in xs.windows(size) {
        let mut total = 0;
        for x in run {
            total += *x;
        }
        sums.push(total);
    }
    sums
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
