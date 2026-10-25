use std::time::Duration;

fn main() {
    let d = Duration::from_millis(1500);
    println!("as_secs: {}", d.as_secs());
    println!("subsec_millis: {}", d.subsec_millis());
    println!("as_millis: {}", d.as_millis());
    println!("as_secs_f64: {}", d.as_secs_f64());
    println!(
        "Debug: {d:?}, {:?}, {:?}",
        Duration::from_micros(250),
        Duration::from_nanos(7)
    );

    let total = Duration::from_secs(90) + Duration::from_millis(250);
    println!("90s + 250ms: {total:?}");
    println!("times 3: {:?}, divided by 4: {:?}", total * 3, total / 4);
    println!("mul_f64(1.5): {:?}", total.mul_f64(1.5));

    // a Duration is never negative
    let (small, big) = (Duration::from_secs(1), Duration::from_secs(2));
    println!("1s checked_sub 2s: {:?}", small.checked_sub(big));
    println!("1s saturating_sub 2s: {:?}", small.saturating_sub(big));
    println!(
        "try_from_secs_f64(-1.0): {:?}",
        Duration::try_from_secs_f64(-1.0)
    );
}
