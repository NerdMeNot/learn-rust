use std::mem::size_of;

fn main() {
    println!(
        "&dyn Fn(i32) -> i32:      {}",
        size_of::<&dyn Fn(i32) -> i32>()
    );
    println!(
        "Box<dyn Fn(i32) -> i32>:  {}",
        size_of::<Box<dyn Fn(i32) -> i32>>()
    );
    println!("fn(i32) -> i32:           {}", size_of::<fn(i32) -> i32>());
    println!("&i32 (for comparison):    {}", size_of::<&i32>());
}
