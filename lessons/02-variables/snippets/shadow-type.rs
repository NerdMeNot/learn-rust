fn main() {
    let speed = 90; // i32
    println!("first: {} bytes", size_of_val(&speed));
    let speed = speed as f64 * 1.6; // a new variable: f64
    println!("second: {} bytes, {speed}", size_of_val(&speed));
}
