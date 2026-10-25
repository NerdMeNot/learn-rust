use std::collections::HashMap;

fn main() {
    // Key on the float's bits: equal bit patterns are equal keys.
    let mut m: HashMap<u64, &str> = HashMap::new();
    m.insert(f64::NAN.to_bits(), "first");
    m.insert(f64::NAN.to_bits(), "second");
    println!(
        "NaN twice: len {}, found: {:?}",
        m.len(),
        m.get(&f64::NAN.to_bits())
    );

    let mut zeros: HashMap<u64, &str> = HashMap::new();
    zeros.insert(0.0_f64.to_bits(), "zero");
    zeros.insert((-0.0_f64).to_bits(), "minus zero");
    println!("0.0 and -0.0: len {}", zeros.len());
}
