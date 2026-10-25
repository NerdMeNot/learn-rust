fn main() {
    println!("7 / 2: {}", 7 / 2);
    println!("7.0 / 2.0: {}", 7.0 / 2.0);
    println!("7 as f64 / 2.0: {}", 7 as f64 / 2.0);

    println!("-7 / 2: {}", -7 / 2);
    println!("-7 % 2: {}", -7 % 2);
    println!("7 % -2: {}", 7 % -2);
    println!("(-7).div_euclid(2): {}", (-7_i32).div_euclid(2));
    println!("(-7).rem_euclid(2): {}", (-7_i32).rem_euclid(2));
    println!("-7 % 5: {}", -7 % 5);
    println!("(-7).rem_euclid(5): {}", (-7_i32).rem_euclid(5));

    // A wrapped index: step back one place in a ring of 5 slots.
    let slot = 0_i32;
    println!("(slot - 1) % 5: {}", (slot - 1) % 5);
    println!("(slot - 1).rem_euclid(5): {}", (slot - 1).rem_euclid(5));
}
