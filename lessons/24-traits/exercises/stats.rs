trait Stats {
    /// The average, or None when there are no numbers.
    fn mean(&self) -> Option<f64>;
    /// The largest minus the smallest, or None when there are no numbers.
    fn spread(&self) -> Option<i64>;
}

// Implement the trait for the slice type [i32], a type you do not own.
// Vec<i32> and arrays can then call these methods too.
impl Stats for [i32] {
    fn mean(&self) -> Option<f64> {
        todo!("add up as i64 so large numbers cannot overflow, then divide")
    }

    fn spread(&self) -> Option<i64> {
        todo!("max and min return Option; the subtraction must not overflow an i32")
    }
}

fn main() {
    let v = vec![2, 4, 9];
    assert_eq!(v.mean(), Some(5.0));
    assert_eq!(v.spread(), Some(7));
    // Arrays and sub-slices work too.
    assert_eq!([10, 20].mean(), Some(15.0));
    // A mean that is not a whole number.
    assert_eq!([1, 2].mean(), Some(1.5));
    assert_eq!(v[1..].spread(), Some(5));
    // Negative numbers.
    assert_eq!([-3, 3].spread(), Some(6));
    // Large numbers: adding them as i32 would overflow.
    assert_eq!([i32::MAX, i32::MAX].mean(), Some(i32::MAX as f64));
    assert_eq!([i32::MIN, i32::MAX].spread(), Some(4_294_967_295));
    // No numbers, no answer.
    let empty: Vec<i32> = vec![];
    assert_eq!(empty.mean(), None);
    assert_eq!(empty.spread(), None);
    println!("all checks passed");
}
