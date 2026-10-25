fn main() {
    // Floats to integers: cut toward zero, saturate at the ends, NaN becomes 0.
    println!("-2.9 as i32: {}", -2.9_f64 as i32);
    println!("1e10 as i32: {}", 1e10_f64 as i32);
    println!("-1e10 as i32: {}", -1e10_f64 as i32);
    println!("300.7 as u8: {}", 300.7_f64 as u8);
    println!("-5.0 as u8: {}", -5.0_f64 as u8);
    println!("NaN as i32: {}", f64::NAN as i32);
    println!("inf as i64: {}", f64::INFINITY as i64);

    // Integers to floats: rounds to the nearest value the float can hold.
    println!("16777216 as f32: {}", 16_777_216_i32 as f32);
    println!("16777217 as f32: {}", 16_777_217_i32 as f32);
    println!("i64::MAX as f64: {}", i64::MAX as f64);
    println!("u64::MAX as f32: {}", u64::MAX as f32);
}
