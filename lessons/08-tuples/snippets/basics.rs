use std::mem::size_of;

fn main() {
    let point = (3, 4);
    println!("point.0: {}, point.1: {}", point.0, point.1);

    // Each slot has its own type.
    let record: (i32, &str, bool) = (7, "seven", true);
    println!("record: {record:?}");
    println!("record.1: {}", record.1);

    // Tuples nest, and each .N goes one level in.
    let nested = ((1, 2), "x");
    println!("nested.0.1: {}", nested.0.1);

    // A tuple changes in place only if the variable is `mut`.
    let mut counter = (0, "hits");
    counter.0 += 10;
    println!("counter: {counter:?}");

    // size_of::<T>() is the size of T in bytes.
    println!("size_of::<(i32, i32)>(): {}", size_of::<(i32, i32)>());
    println!("size_of::<(u8, u32)>():  {}", size_of::<(u8, u32)>());
}
