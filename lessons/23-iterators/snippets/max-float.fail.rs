fn main() {
    let temps: [f64; 3] = [21.5, 19.0, 23.25];
    let hottest = temps.iter().max();
    println!("{hottest:?}");
}
