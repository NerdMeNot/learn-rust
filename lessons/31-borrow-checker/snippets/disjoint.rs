use std::mem;

fn main() {
    // Swapping: there is a method.
    let mut v = vec![1, 2, 3, 4];
    v.swap(0, 3);
    println!("swap: {v:?}");

    // Two &mut that cannot overlap: split the list in two.
    let mut v = vec![1, 2, 3, 4];
    let (left, right) = v.split_at_mut(2);
    mem::swap(&mut left[0], &mut right[1]);
    println!("split_at_mut: {v:?}");

    // Two &mut chosen by index: the check happens at run time.
    let mut v = vec![1, 2, 3, 4];
    if let Ok([a, b]) = v.get_disjoint_mut([0, 3]) {
        mem::swap(a, b);
    }
    println!("get_disjoint_mut: {v:?}");
    println!("same index: {}", v.get_disjoint_mut([1, 1]).is_err());
}
