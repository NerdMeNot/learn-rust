fn main() {
    let mut v = vec![10, 20, 30, 40];

    // 1. Swap two elements: the Vec does it for you.
    v.swap(0, 3);
    println!("after v.swap(0, 3): {v:?}");

    // 2. Numbers are copied out of the Vec, so read first and write after.
    let (i, j) = (1, 2);
    let moved = v[i];
    v[i] = 0;
    v[j] += moved;
    println!("after moving v[1] into v[2]: {v:?}");

    // 3. split_at_mut gives two halves that cannot overlap.
    let (left, right) = v.split_at_mut(2);
    left[0] += right[1];
    right[0] += 1;
    println!("after split_at_mut(2): {v:?}");
}
