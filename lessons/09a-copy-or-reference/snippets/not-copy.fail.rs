// Compiles only for types that are `Copy`.
fn is_copy<T: Copy>(_name: &str) {}

fn main() {
    is_copy::<i32>("i32");
    is_copy::<String>("String");
    is_copy::<Vec<i32>>("Vec<i32>");
    is_copy::<&mut i32>("&mut i32");
}
