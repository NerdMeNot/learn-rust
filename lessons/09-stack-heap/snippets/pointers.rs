use std::ptr;

fn main() {
    let x = 42;
    let r = &x; // a reference to x: the address of x, not a copy
    println!("r points at x: {}", ptr::eq(r, &x));
    println!("*r: {}", *r);

    let b = Box::new(x); // allocates 4 bytes on the heap and copies x into them
    println!("*b: {}", *b);
    println!("b points at x: {}", ptr::eq(&*b, &x));

    println!("size of x: {} bytes", size_of_val(&x));
    println!("size of r: {} bytes", size_of_val(&r));
    println!("size of b: {} bytes", size_of_val(&b));
    println!("size of what b points at: {} bytes", size_of_val(&*b));
}
