use std::mem::align_of;
use std::ptr;

fn main() {
    let mut scores = [10, 20, 30];

    // creating raw pointers is safe, from an array or from nothing
    let first: *const i32 = scores.as_ptr();
    let nothing: *const i32 = ptr::null();
    println!(
        "first is null: {}, nothing is null: {}",
        first.is_null(),
        nothing.is_null()
    );
    println!(
        "first is aligned: {}",
        (first as usize).is_multiple_of(align_of::<i32>())
    );

    // wrapping_add is safe: it makes a pointer and promises nothing about it
    let end = first.wrapping_add(3);
    // SAFETY: both pointers come from `scores`; `end` is one past its last element
    let gap = unsafe { end.offset_from(first) };
    println!("end - first, in elements: {gap}");

    // reading and writing through a pointer are not safe
    let last: *mut i32 = &raw mut scores[2];
    // SAFETY: `last` points at scores[2], which is alive, initialised and not borrowed
    unsafe {
        *last += 1;
        println!("last through the pointer: {}", *last);
    }
    println!("scores: {scores:?}");
}
