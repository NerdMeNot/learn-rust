// With a tuple, nothing says which slot is which.
fn aspect(size: (u32, u32)) -> u32 {
    size.0 / size.1
}

// A struct names them (the Structs lesson covers this).
struct Size {
    width: u32,
    height: u32,
}

fn aspect_named(size: Size) -> u32 {
    size.width / size.height
}

fn main() {
    // Width first, height second? The compiler cannot tell this is swapped.
    println!("aspect((16, 9)): {}", aspect((16, 9)));
    println!("aspect((9, 16)): {}", aspect((9, 16)));

    let size = Size {
        width: 16,
        height: 9,
    };
    println!("aspect_named: {}", aspect_named(size));
}
