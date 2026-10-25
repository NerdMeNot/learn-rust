fn main() {
    let mut inclusive = 0;
    for _ in 0..=255u8 {
        inclusive += 1;
    }
    let mut exclusive = 0;
    for _ in 0..255u8 {
        exclusive += 1;
    }
    println!("0..=255u8 runs: {inclusive}");
    println!("0..255u8 runs: {exclusive}");
}
