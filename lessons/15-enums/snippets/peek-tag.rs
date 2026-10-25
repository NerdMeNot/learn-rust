// The fields are read through raw pointers, which the compiler cannot see.
#![allow(dead_code)]

// With #[repr(u8)] the layout is fixed by the language: a one-byte tag first,
// then the variant's fields. That lets us look at the bytes.
#[repr(u8)]
enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

fn tag(s: &Shape) -> u8 {
    // unsafe: we promise that byte 0 of a Shape is inside the value (it is)
    unsafe { *(s as *const Shape as *const u8) }
}

fn f64_at(s: &Shape, offset: usize) -> f64 {
    // unsafe: we promise that there is an f64 stored at this offset
    unsafe { *((s as *const Shape as *const u8).add(offset) as *const f64) }
}

fn main() {
    println!("size of Shape: {} bytes", size_of::<Shape>());

    let circle = Shape::Circle(1.0);
    println!(
        "Circle(1.0): tag {}, f64 at byte 8: {}",
        tag(&circle),
        f64_at(&circle, 8)
    );

    let rect = Shape::Rect(2.0, 3.0);
    println!(
        "Rect(2.0, 3.0): tag {}, f64 at byte 8: {}, f64 at byte 16: {}",
        tag(&rect),
        f64_at(&rect, 8),
        f64_at(&rect, 16)
    );
}
