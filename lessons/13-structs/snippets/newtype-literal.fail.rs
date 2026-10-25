struct Meters(f64);

fn describe(distance: Meters) -> String {
    format!("{} meters", distance.0)
}

fn main() {
    println!("{}", describe(1200.0));
}
