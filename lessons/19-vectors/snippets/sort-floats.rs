fn main() {
    let nan = f64::NAN;
    println!("NaN < 1.0:  {}", nan < 1.0);
    println!("NaN > 1.0:  {}", nan > 1.0);
    println!("NaN == NaN: {}", nan == nan);
    println!("partial_cmp: {:?}", nan.partial_cmp(&1.0));

    let mut temps = vec![21.5, f64::NAN, 19.0, -0.0, 0.0, 23.25];
    temps.sort_by(|a, b| a.total_cmp(b));
    println!("total_cmp: {temps:?}");
}
