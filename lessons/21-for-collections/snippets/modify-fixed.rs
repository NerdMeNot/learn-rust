fn main() {
    // 1. Remove items: retain keeps the ones for which the closure says true.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|x| x % 2 != 0);
    println!("retain: {v:?}");

    // 2. Collect what to change first; the loop over `&v` has ended before the change.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let mut doomed = Vec::new();
    for (i, x) in v.iter().enumerate() {
        if x % 2 == 0 {
            doomed.push(i);
        }
    }
    for i in doomed.into_iter().rev() {
        v.remove(i);
    }
    println!("collect, then apply: {v:?}");

    // 3. Loop over positions. The range is made once, so no borrow of `v` lasts across the body.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    for i in (0..v.len()).rev() {
        if v[i] % 2 == 0 {
            v.remove(i);
        }
    }
    println!("positions, backwards: {v:?}");

    // Changing the items is not a problem: `&mut` is the one writer.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    for x in &mut v {
        if *x % 2 == 0 {
            *x = 0;
        }
    }
    println!("&mut: {v:?}");
}
