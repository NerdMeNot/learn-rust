use std::any::type_name;
use std::ops::Add;

// `T: Add` says two Ts can be added. The result type is T::Output.
fn sum_type<T: Add>() -> String {
    format!(
        "T = {:<5} T + T = {}",
        type_name::<T>(),
        type_name::<T::Output>()
    )
}

fn main() {
    println!("{}", sum_type::<i32>());
    println!("{}", sum_type::<f64>());
    println!("{}", sum_type::<&i32>());
}
