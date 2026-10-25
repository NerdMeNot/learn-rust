#[derive(Debug)]
enum Level {
    Low = 1,
    High = 10,
}

fn main() {
    let n = 10;
    let level = n as Level;
    println!("{level:?}");
}
