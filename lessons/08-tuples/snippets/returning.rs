fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    // Unpack into two names.
    let (q, r) = divmod(17, 5);
    println!("q: {q}, r: {r}");

    // Or keep the pair whole and read its slots.
    let pair = divmod(17, 5);
    println!("pair: {pair:?}, pair.0: {}", pair.0);

    // `_` ignores a slot.
    let (_, rem) = divmod(40, 7);
    println!("rem: {rem}");
}
