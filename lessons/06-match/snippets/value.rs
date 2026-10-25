fn main() {
    for n in [0, 3, 7, 12] {
        let size = match n {
            0 => "none",
            1..=5 => "few",
            6..=10 => "several",
            _ => "many",
        };
        println!("{n} -> {size}");
    }
}
