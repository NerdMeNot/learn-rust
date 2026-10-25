fn main() {
    println!(
        "f32 size: {}, f64 size: {}",
        size_of::<f32>(),
        size_of::<f64>()
    );
    println!("f32::MAX: {:e}", f32::MAX);
    println!("f64::MAX: {:e}", f64::MAX);
    println!("f64::EPSILON: {:e}", f64::EPSILON);

    println!("0.1 + 0.2: {}", 0.1 + 0.2);
    println!("0.1 + 0.2 == 0.3: {}", 0.1 + 0.2 == 0.3);
    println!("close enough: {}", (0.1_f64 + 0.2 - 0.3).abs() < 1e-9);

    let zero = 0.0_f64;
    println!("1.0 / 0.0: {}", 1.0 / zero);
    println!("-1.0 / 0.0: {}", -1.0 / zero);
    println!("0.0 / 0.0: {}", zero / zero);

    let nan = f64::NAN;
    println!("NaN == NaN: {}", nan == nan);
    println!("NaN < 1.0: {}", nan < 1.0);
    println!("NaN > 1.0: {}", nan > 1.0);
    println!("NaN.is_nan(): {}", nan.is_nan());
    println!("1.0.partial_cmp(&2.0): {:?}", 1.0_f64.partial_cmp(&2.0));
    println!("1.0.partial_cmp(&NaN): {:?}", 1.0_f64.partial_cmp(&nan));
    println!("1.0.total_cmp(&NaN): {:?}", 1.0_f64.total_cmp(&nan));
    println!("0.0 == -0.0: {}", 0.0_f64 == -0.0);
    println!("0.0.total_cmp(&-0.0): {:?}", 0.0_f64.total_cmp(&-0.0));
}
