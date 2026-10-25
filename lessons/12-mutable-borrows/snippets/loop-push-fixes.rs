fn main() {
    // Fix 1: loop over the positions that exist now, and look each one up.
    let mut v = vec![1, 2, 3];
    let count = v.len();
    for i in 0..count {
        if v[i] == 2 {
            v.push(20);
        }
    }
    println!("index loop: {v:?}");

    // Fix 2: collect what to add in a second list, then add it after the loop.
    let mut w = vec![1, 2, 3];
    let mut extra = Vec::new();
    for x in &w {
        if *x == 2 {
            extra.push(20);
        }
    }
    w.extend(extra);
    println!("second list: {w:?}");
}
