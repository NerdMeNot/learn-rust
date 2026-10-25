use std::mem::size_of;

fn main() {
    println!(
        "i8    {:>2} byte  {} to {}",
        size_of::<i8>(),
        i8::MIN,
        i8::MAX
    );
    println!(
        "i16   {:>2} bytes {} to {}",
        size_of::<i16>(),
        i16::MIN,
        i16::MAX
    );
    println!(
        "i32   {:>2} bytes {} to {}",
        size_of::<i32>(),
        i32::MIN,
        i32::MAX
    );
    println!(
        "i64   {:>2} bytes {} to {}",
        size_of::<i64>(),
        i64::MIN,
        i64::MAX
    );
    println!(
        "i128  {:>2} bytes {} to {}",
        size_of::<i128>(),
        i128::MIN,
        i128::MAX
    );
    println!(
        "u8    {:>2} byte  {} to {}",
        size_of::<u8>(),
        u8::MIN,
        u8::MAX
    );
    println!(
        "u16   {:>2} bytes {} to {}",
        size_of::<u16>(),
        u16::MIN,
        u16::MAX
    );
    println!(
        "u32   {:>2} bytes {} to {}",
        size_of::<u32>(),
        u32::MIN,
        u32::MAX
    );
    println!(
        "u64   {:>2} bytes {} to {}",
        size_of::<u64>(),
        u64::MIN,
        u64::MAX
    );
    println!(
        "u128  {:>2} bytes {} to {}",
        size_of::<u128>(),
        u128::MIN,
        u128::MAX
    );
    println!(
        "isize {:>2} bytes {} to {}",
        size_of::<isize>(),
        isize::MIN,
        isize::MAX
    );
    println!(
        "usize {:>2} bytes {} to {}",
        size_of::<usize>(),
        usize::MIN,
        usize::MAX
    );
}
