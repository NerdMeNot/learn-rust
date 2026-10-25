use std::mem::{align_of, offset_of, size_of};

// The fields are never read, only measured.
#[allow(dead_code)]
struct Plain {
    a: u8,
    b: u32,
    c: u8,
}

#[allow(dead_code)]
#[repr(C)]
struct CLayout {
    a: u8,
    b: u32,
    c: u8,
}

fn main() {
    println!("u8: size {}, align {}", size_of::<u8>(), align_of::<u8>());
    println!(
        "u32: size {}, align {}",
        size_of::<u32>(),
        align_of::<u32>()
    );
    println!(
        "u64: size {}, align {}",
        size_of::<u64>(),
        align_of::<u64>()
    );
    println!("fields add up to {} bytes", 1 + 4 + 1);

    println!(
        "Plain:   size {}, align {}",
        size_of::<Plain>(),
        align_of::<Plain>()
    );
    println!(
        "  offsets: a {}, b {}, c {}",
        offset_of!(Plain, a),
        offset_of!(Plain, b),
        offset_of!(Plain, c)
    );

    println!(
        "CLayout: size {}, align {}",
        size_of::<CLayout>(),
        align_of::<CLayout>()
    );
    println!(
        "  offsets: a {}, b {}, c {}",
        offset_of!(CLayout, a),
        offset_of!(CLayout, b),
        offset_of!(CLayout, c)
    );
}
