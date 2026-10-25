const LIMIT: u32 = 1_000; // a constant: the type is required

fn half(x: f64) -> f64 {
    x / 2.0
}

fn main() {
    let name: &str = "Ada"; // type after the name
    let mut visits = 0u8; // `u8` is a suffix: this 0 is a u8
    visits += 1;
    let (mask, ratio) = (0xff, 2.5f32); // two names, one pattern
    let letter = 'R'; // single quotes: one character
    let total = LIMIT as f64 + half(5.0); // `as` converts
    let parsed = "42".parse::<i32>().unwrap(); // `unwrap`: stop if not a number

    println!("name={name} visits={visits}");
    println!("mask={mask} ratio={ratio} letter={letter}");
    println!("limit={LIMIT} total={total}");
    println!("parsed={parsed}");
}
