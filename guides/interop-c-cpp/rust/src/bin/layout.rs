use std::mem::{offset_of, size_of};

#[allow(dead_code)]
struct Reordered {
    id: u8,
    value: f64,
    flag: u8,
}

fn main() {
    println!("Rust, Reading: size {}, id at {}, value at {}, flag at {}",
        size_of::<bridge::Reading>(),
        offset_of!(bridge::Reading, id),
        offset_of!(bridge::Reading, value),
        offset_of!(bridge::Reading, flag));
    println!("default repr, same fields: size {}", size_of::<Reordered>());
}
