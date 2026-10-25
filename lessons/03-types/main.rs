fn main() {
    let a: i32 = 7; // 32-bit signed integer
    let b: i64 = 10; // 64-bit signed integer
    let price: f64 = 2.5; // 64-bit float
    let ok: bool = true;

    let total = a as i64 + b; // convert with `as`
    let cost = a as f64 * price;
    println!("{total} {cost} {ok}");
}
