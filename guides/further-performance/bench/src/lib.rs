/// Scrambles the bits of a number `rounds` times (xorshift). The loop has no short formula.
pub fn mix(mut x: u64, rounds: u32) -> u64 {
    for _ in 0..rounds {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
    }
    x
}
