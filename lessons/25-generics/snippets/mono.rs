use std::any::type_name;
use std::mem::size_of;

fn describe<T>() {
    println!(
        "T = {:<30} size_of::<T>() = {}",
        type_name::<T>(),
        size_of::<T>()
    );
}

fn largest<T: PartialOrd>(items: &[T]) -> &T {
    let mut best = &items[0];
    for item in items {
        if item > best {
            best = item;
        }
    }
    best
}

fn main() {
    // The same function body, once per T, and each copy knows its T.
    describe::<u8>();
    describe::<u64>();
    describe::<String>();
    describe::<&str>();
    describe::<Option<u32>>();

    let for_ints = largest::<i32> as *const ();
    let for_floats = largest::<f64> as *const ();
    println!(
        "largest::<i32> and largest::<f64> are different functions: {}",
        for_ints != for_floats
    );
}
