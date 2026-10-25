// Each call to `depth` gets its own frame, and none of them returns
// until the one below it does. The frames pile up.
fn depth(n: u64) -> u64 {
    if n == 0 { 0 } else { 1 + depth(n - 1) }
}

fn main() {
    println!("depth(1_000): {}", depth(1_000));
    println!("depth(100_000_000): {}", depth(100_000_000));
}
