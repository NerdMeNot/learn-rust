use std::mem::{align_of, size_of};

// We only measure these structs, so the fields are never read.
// #[repr(C)] keeps the fields in the order written, as a C compiler would.
#[allow(dead_code)]
#[repr(C)]
struct MixedC {
    a: u8,
    n: u32,
    b: u8,
}

#[allow(dead_code)]
#[repr(C)]
struct SmallFirstC {
    a: u8,
    b: u8,
    n: u32,
}

fn main() {
    println!("            size  align");
    println!(
        "u32         {:>4}  {:>5}",
        size_of::<u32>(),
        align_of::<u32>()
    );
    println!(
        "[u8; 16]    {:>4}  {:>5}",
        size_of::<[u8; 16]>(),
        align_of::<[u8; 16]>()
    );
    println!(
        "MixedC      {:>4}  {:>5}",
        size_of::<MixedC>(),
        align_of::<MixedC>()
    );
    println!(
        "SmallFirstC {:>4}  {:>5}",
        size_of::<SmallFirstC>(),
        align_of::<SmallFirstC>()
    );
}
