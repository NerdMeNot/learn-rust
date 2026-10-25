fn main() {
    let prices = [10, 20, 30];
    let p = prices.as_ptr();
    // SAFETY: `p` points at the first of three elements of `prices`, which is alive, and 1 < 3
    let second = unsafe { *p.add(1) };
    println!("second: {second}");
}
