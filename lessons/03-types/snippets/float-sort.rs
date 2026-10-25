fn main() {
    let mut readings = vec![2.5, f64::NAN, 0.5, -1.0];
    readings.sort_by(|a, b| a.total_cmp(b));
    println!("sorted: {readings:?}");
}
