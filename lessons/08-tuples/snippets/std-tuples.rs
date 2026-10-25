fn main() {
    // split_at: two slices from one.
    let (left, right) = [1, 2, 3, 4].split_at(1);
    println!("split_at(1): {left:?} and {right:?}");

    // overflowing_add: the wrapped sum, and whether it overflowed.
    let (sum, overflowed) = 250u8.overflowing_add(10);
    println!("250u8.overflowing_add(10): sum: {sum}, overflowed: {overflowed}");

    // Both are plain tuples: keep one whole if you like.
    let result = 250u8.overflowing_add(10);
    println!("result.0: {}, result.1: {}", result.0, result.1);
}
