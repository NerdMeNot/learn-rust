fn main() {
    let x = 5;
    let p = &x as *const i32;
    println!("{}", *p);
}
