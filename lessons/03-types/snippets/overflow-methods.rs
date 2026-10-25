fn main() {
    let level: u8 = 250;
    println!("checked_add(10): {:?}", level.checked_add(10));
    println!("checked_add(5): {:?}", level.checked_add(5));
    println!("wrapping_add(10): {}", level.wrapping_add(10));
    println!("saturating_add(10): {}", level.saturating_add(10));
    println!("overflowing_add(10): {:?}", level.overflowing_add(10));

    // The same families exist for -, *, /, pow and neg.
    println!("0u8.saturating_sub(1): {}", 0u8.saturating_sub(1));
    println!("i32::MIN.checked_abs(): {:?}", i32::MIN.checked_abs());
    println!("i32::MIN.wrapping_abs(): {}", i32::MIN.wrapping_abs());
    println!("i32::MIN.unsigned_abs(): {}", i32::MIN.unsigned_abs());
    println!("10i32.checked_pow(10): {:?}", 10i32.checked_pow(10));
}
