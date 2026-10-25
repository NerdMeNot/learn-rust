fn main() {
    let n = 5;
    let sign = match n {
        x if x < 0 => "negative",
        x if x >= 0 => "not negative",
    };
    println!("{sign}");
}
