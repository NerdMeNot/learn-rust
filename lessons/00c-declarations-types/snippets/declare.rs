const MAX_LEVEL: u32 = 99;
static GREETING: &str = "hello";

fn main() {
    let score = 10; // type inferred
    let mut lives = 3; // can change
    let ratio: f64 = 0.5; // type written
    lives -= 1;

    println!("score: {score}, lives: {lives}, ratio: {ratio}");
    println!("max level: {MAX_LEVEL}, greeting: {GREETING}");
}
