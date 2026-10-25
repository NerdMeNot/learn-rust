fn main() {
    let n = 3;
    let label = match n {
        0 => "zero",
        _ => n,
    };
    println!("{label}");
}
