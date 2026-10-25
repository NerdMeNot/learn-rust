use std::mem::size_of;

/// Bytes taken by each type on a 64-bit machine, in this order:
/// u8, i32, (i32, i32), [u16; 5], &str, Box<String>, Vec<u8>
fn predictions() -> [usize; 7] {
    // A Box is one pointer, however big the String it points at is.
    [1, 4, 8, 10, 16, 8, 24]
}

fn main() {
    let actual = [
        size_of::<u8>(),
        size_of::<i32>(),
        size_of::<(i32, i32)>(),
        size_of::<[u16; 5]>(),
        size_of::<&str>(),
        size_of::<Box<String>>(),
        size_of::<Vec<u8>>(),
    ];
    assert_eq!(predictions(), actual);
    println!("all checks passed");
}
