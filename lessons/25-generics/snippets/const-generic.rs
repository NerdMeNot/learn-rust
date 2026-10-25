use std::any::type_name;

// N is a number the compiler knows, filled in from the array's type.
fn sum<const N: usize>(values: [i32; N]) -> i32 {
    println!("  an array of {N}");
    values.iter().sum()
}

struct Buffer<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Buffer<N> {
    fn new() -> Buffer<N> {
        Buffer { data: [0; N] }
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    println!("sum: {}", sum([1, 2, 3]));
    println!("sum: {}", sum([10, 20, 30, 40, 50]));
    println!("sum of none: {}", sum([]));
    println!("capacity: {}", Buffer::<16>::new().capacity());
    println!("type of [i32; 3]: {}", type_name::<[i32; 3]>());
    println!("type of [i32; 4]: {}", type_name::<[i32; 4]>());
}
