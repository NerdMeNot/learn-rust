fn main() {
    let n = 7;
    let size = match n {
        0 => "none",
        1..=5 => "few",
    };
    println!("{size}");
}
