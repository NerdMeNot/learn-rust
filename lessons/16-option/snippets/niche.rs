use std::mem::size_of;
use std::num::NonZeroU32;

fn main() {
    // A reference is never null, so None can use the all-zero address and no tag is needed.
    println!("size of &i32: {}", size_of::<&i32>());
    println!("size of Option<&i32>: {}", size_of::<Option<&i32>>());
    println!("size of Box<i32>: {}", size_of::<Box<i32>>());
    println!(
        "size of Option<Box<i32>>: {}",
        size_of::<Option<Box<i32>>>()
    );
    println!("size of NonZeroU32: {}", size_of::<NonZeroU32>());
    println!(
        "size of Option<NonZeroU32>: {}",
        size_of::<Option<NonZeroU32>>()
    );

    // Every bit pattern of an i32 is a number, so Option<i32> needs room for a tag.
    println!("size of i32: {}", size_of::<i32>());
    println!("size of Option<i32>: {}", size_of::<Option<i32>>());
    println!("size of *const i32: {}", size_of::<*const i32>());
    println!(
        "size of Option<*const i32>: {}",
        size_of::<Option<*const i32>>()
    );
}
