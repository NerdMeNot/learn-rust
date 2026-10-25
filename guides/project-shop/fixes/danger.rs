//! An `unsafe` block in a crate that inherits the workspace lints.
fn main() {
    let n = 7;
    let p = &n as *const i32;
    println!("{}", unsafe { *p });
}
