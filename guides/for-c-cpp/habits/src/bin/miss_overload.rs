fn area(side: f64) -> f64 {
    side * side
}

fn area(width: f64, height: f64) -> f64 {
    width * height
}

fn main() {
    println!("{} {}", area(2.0), area(2.0, 3.0));
}
