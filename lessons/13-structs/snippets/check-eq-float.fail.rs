#[derive(PartialEq, Eq)]
struct Reading {
    celsius: f64,
}

fn main() {
    let r = Reading { celsius: 1.0 };
    println!("{}", r == r);
}
