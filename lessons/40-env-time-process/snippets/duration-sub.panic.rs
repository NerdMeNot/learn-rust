use std::time::Duration;

fn main() {
    let (small, big) = (Duration::from_secs(1), Duration::from_secs(2));
    println!("about to subtract");
    println!("{:?}", small - big);
}
