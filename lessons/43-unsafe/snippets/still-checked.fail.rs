fn main() {
    let mut scores = vec![1, 2, 3];
    unsafe {
        let p = scores.as_ptr();
        let last = *p.add(2); // needs unsafe
        let first = &mut scores[0]; // an ordinary reference
        let second = &mut scores[1]; // a second one: still checked
        *first += last;
        *second += last;
    }
    println!("{scores:?}");
}
