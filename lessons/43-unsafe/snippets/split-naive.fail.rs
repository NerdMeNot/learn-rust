fn split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid <= v.len());
    (&mut v[..mid], &mut v[mid..])
}

fn main() {
    let mut v = [1, 2, 3, 4];
    let (a, b) = split_at_mut(&mut v, 2);
    a[0] = 10;
    b[0] = 30;
    println!("{v:?}");
}
