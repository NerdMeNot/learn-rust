use std::collections::HashMap;

fn main() {
    let mut readings: HashMap<f64, &str> = HashMap::new();
    readings.insert(20.5, "warm");
    println!("{}", readings.len());
}
