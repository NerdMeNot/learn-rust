fn depth(n: u64) -> u64 {
    if n == 0 { 0 } else { 1 + depth(n - 1) }
}

fn main() {
    println!("depth(1_000): {}", depth(1_000));
    println!("depth(100_000_000): {}", depth(100_000_000));
}
