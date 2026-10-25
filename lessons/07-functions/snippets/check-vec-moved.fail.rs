fn take(xs: Vec<i32>) -> usize {
    xs.len()
}

fn main() {
    let xs = vec![1, 2, 3];
    let n = take(xs);
    println!("{n} {}", xs.len());
}
