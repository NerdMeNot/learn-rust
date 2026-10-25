fn main() {
    let mut fs: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for i in 0..3 {
        fs.push(Box::new(|| i));
    }
    for f in &fs {
        print!("{} ", f());
    }
    println!();
}
