fn main() {
    let x: i32 = 42;
    let r: &i32 = &x;

    println!(
        "x is {} bytes, r is {} bytes",
        size_of::<i32>(),
        size_of::<&i32>()
    );
    println!("*r: {}", *r);
    // ptr::eq asks whether two references hold the same address.
    println!("r points at x itself: {}", std::ptr::eq(r, &x));

    // A Vec is a 24-byte header plus its elements on the heap.
    // A reference to the Vec is one address, whatever the Vec holds.
    let v: Vec<i32> = vec![1, 2, 3];
    let rv: &Vec<i32> = &v;
    println!("v: len {}, capacity {}", v.len(), v.capacity());
    println!(
        "v is {} bytes, rv is {} bytes",
        size_of::<Vec<i32>>(),
        size_of::<&Vec<i32>>()
    );
    println!("rv points at v itself: {}", std::ptr::eq(rv, &v));
    println!("same elements on the heap: {}", rv.as_ptr() == v.as_ptr());
}
