// Reverse the items of `xs` in place, without making a new Vec.
// (Do not call `xs.reverse()`: the point is to write it with `swap`.)
fn reverse_in_place(xs: &mut [i32]) {
    todo!("swap item i with item len - 1 - i, for the first half of the positions")
}

fn main() {
    let mut even = vec![1, 2, 3, 4];
    reverse_in_place(&mut even);
    assert_eq!(even, vec![4, 3, 2, 1]);

    let mut odd = vec![1, 2, 3];
    reverse_in_place(&mut odd);
    assert_eq!(odd, vec![3, 2, 1]);

    let mut empty: Vec<i32> = Vec::new();
    reverse_in_place(&mut empty);
    assert!(empty.is_empty());

    let mut one = [9];
    reverse_in_place(&mut one);
    assert_eq!(one, [9]);

    // Only the middle of the list is a slice here; the ends must not move.
    let mut part = vec![1, 2, 3, 4, 5];
    reverse_in_place(&mut part[1..4]);
    assert_eq!(part, vec![1, 4, 3, 2, 5]);
    println!("all checks passed");
}
