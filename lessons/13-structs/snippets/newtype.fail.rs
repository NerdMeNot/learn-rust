struct Meters(f64);
struct Feet(f64);

fn describe(distance: Meters) -> String {
    format!("{} meters", distance.0)
}

fn main() {
    let altitude = Feet(1200.0);
    println!("{}", describe(altitude));
}
