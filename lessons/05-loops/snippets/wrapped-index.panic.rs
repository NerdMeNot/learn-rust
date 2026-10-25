fn main() {
    let items = [5, 6, 7];
    // what `i -= 1` gives at 0 when overflow checks are off: wrapping_sub always wraps
    let i = std::hint::black_box(0usize).wrapping_sub(1);
    println!("i: {i}");
    println!("items[i] = {}", items[i]);
}
