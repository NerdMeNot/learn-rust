use std::mem::{size_of, size_of_val};

fn main() {
    let factor = 3;
    let mut count = 0;
    let name = String::from("ferris");
    println!("name: len {}, capacity {}", name.len(), name.capacity());

    let no_captures = |x: i32| x + 1;
    let reads_factor = |x: i32| x * factor;
    let bumps_count = || count += 1;
    let owns_factor = move |x: i32| x * factor;
    let owns_name = move || name.len();

    println!(
        "size of a closure that captures nothing: {}",
        size_of_val(&no_captures)
    );
    println!(
        "reads factor (shared ref):    {}",
        size_of_val(&reads_factor)
    );
    println!(
        "bumps count (mutable ref):    {}",
        size_of_val(&bumps_count)
    );
    println!(
        "owns factor (an i32):         {}",
        size_of_val(&owns_factor)
    );
    println!("owns name (a String):         {}", size_of_val(&owns_name));
    println!(
        "for comparison, &i32: {}, i32: {}, String: {}",
        size_of::<&i32>(),
        size_of::<i32>(),
        size_of::<String>()
    );
}
