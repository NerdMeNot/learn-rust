use std::any::{Any, TypeId};

// `T: 'static` means T holds no borrowed data. TypeId works for such types.
fn same_type<A: 'static, B: 'static>() -> bool {
    TypeId::of::<A>() == TypeId::of::<B>()
}

fn is_a<T: 'static>(value: &dyn Any) -> bool {
    value.is::<T>()
}

fn main() {
    println!("same_type::<i32, i32>(): {}", same_type::<i32, i32>());
    println!("same_type::<i32, u8>(): {}", same_type::<i32, u8>());
    println!("is_a::<i32>(&3i32): {}", is_a::<i32>(&3i32));
    println!("is_a::<i32>(&\"x\"): {}", is_a::<i32>(&"x"));
}
