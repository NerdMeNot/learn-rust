/// `Some(slot)` when `i` names a slot in a table of `len` slots (numbered from
/// 0), otherwise `None`.
fn checked_index(i: i32, len: u8) -> Option<u8> {
    todo!("u8::try_from(i) is an Err when i does not fit in a u8; .ok() makes it an Option")
}

fn main() {
    assert_eq!(checked_index(0, 3), Some(0));
    assert_eq!(checked_index(2, 3), Some(2));
    assert_eq!(checked_index(3, 3), None);
    assert_eq!(checked_index(-1, 3), None);
    assert_eq!(checked_index(i32::MIN, 3), None);
    assert_eq!(checked_index(i32::MAX, 3), None);
    // An empty table has no valid slot.
    assert_eq!(checked_index(0, 0), None);
    // `i as u8` would wrap 256 to 0 and 300 to 44, and say Some.
    assert_eq!(checked_index(256, 10), None);
    assert_eq!(checked_index(300, 100), None);
    assert_eq!(checked_index(254, 255), Some(254));
    assert_eq!(checked_index(255, 255), None);

    println!("all checks passed");
}
