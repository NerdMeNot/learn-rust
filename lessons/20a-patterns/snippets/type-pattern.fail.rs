fn main() {
    let value = 5i32;
    match value {
        // Java: `case Integer n ->`. A Rust pattern has no type test.
        _n: i32 => println!("an i32"),
    }
}
