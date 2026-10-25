// Backs the claim in "Why this macro uses `=>`": a `literal` key may be followed by `:`.
macro_rules! pair {
    ($key:literal : $value:expr) => {
        ($key, $value)
    };
}

fn main() {
    assert_eq!(pair!("Ada": 36), ("Ada", 36));
    println!("ok");
}
