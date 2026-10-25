fn main() {
    let mut level: u8 = 250;
    for step in 1..=10 {
        level += 1;
        println!("step {step}: level {level}");
    }
}
