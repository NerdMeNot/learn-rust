fn main() {
    let sum = 0.1 + 0.2;
    println!("{{}}:   {}", sum);
    println!("{{:.1}}: {:.1}", sum);
    println!(
        "round trip: {}",
        sum.to_string().parse::<f64>().unwrap() == sum
    );
    println!("{} {} {}", f64::NAN, f64::INFINITY, -f64::INFINITY);
    println!("{} {:?}", -0.0, -0.0);
    println!("{{:e}}: {:e} {:e}", 1234.5, 0.00012);

    for x in [0.5, 1.5, 2.5, 3.5, -0.5] {
        print!("{:.0} ", x);
    }
    println!("<- {{:.0}}");
    for x in [0.125, 0.375, 2.675, 1.005] {
        print!("{:.2} ", x);
    }
    println!("<- {{:.2}}");
    println!("2.675 is really {:.20}", 2.675);
    println!("{}", 1234567.5);
}
