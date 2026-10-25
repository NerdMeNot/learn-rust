fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);

    let mut w = vec![1, 2, 3]; // vec! makes a Vec with these values
    w.push(4);

    let part: &[i32] = &w[1..3]; // a slice: a borrowed window, no copy
    println!("{v:?} {w:?} {part:?} len={}", w.len());
}
