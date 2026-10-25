fn total(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

fn main() {
    let mut xs = vec![1, 2, 3];
    let view: &[i32] = &xs;
    println!("{:?} {}", view, total(view));
    xs.push(4);
    println!("{:?}", xs);
}
