use std::mem::size_of;
use std::ptr;

fn main() {
    let v = vec![1, 2, 3];
    let copy = v.clone(); // a new Vec with its own buffer
    let view = &v[..]; // a pointer and a length into v's buffer

    println!("size_of Vec<i32>: {} bytes", size_of::<Vec<i32>>());
    println!("size_of &[i32]:   {} bytes", size_of::<&[i32]>());
    println!(
        "v and copy share a buffer: {}",
        ptr::eq(v.as_ptr(), copy.as_ptr())
    );
    println!(
        "v and view share a buffer: {}",
        ptr::eq(v.as_ptr(), view.as_ptr())
    );

    let a = 5;
    let b = a;
    let r1 = &a;
    let r2 = &a;
    println!("a and b are one variable:  {}", ptr::eq(&a, &b));
    println!("r1 and r2 are one place:   {}", ptr::eq(r1, r2));
}
