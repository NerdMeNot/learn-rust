union Bits {
    float: f32,
    int: u32,
}

static mut HITS: u32 = 0;

/// # Safety
/// `p` must point to a valid, initialized `i32`.
unsafe fn read(p: *const i32) -> i32 {
    // SAFETY: the caller promised `p` is valid for reads
    unsafe { *p }
}

/// # Safety
/// Implement this only for types where every all-zero bit pattern is a valid value.
unsafe trait ZeroIsValid {}

// SAFETY: the integer 0 is a valid u32
unsafe impl ZeroIsValid for u32 {}

fn zeroed<T: ZeroIsValid>() -> T {
    // SAFETY: the trait's contract says all-zero bits are a valid T
    unsafe { std::mem::zeroed() }
}

fn main() {
    let x = 7;
    let p = &raw const x; // making a raw pointer is safe
    println!("p is null: {}", p.is_null());
    // SAFETY: `p` points at `x`, which is alive, initialized and not mutated
    let first = unsafe { *p };
    println!("1 dereference: {first}");
    // SAFETY: the same `p` meets `read`'s condition
    let second = unsafe { read(p) };
    println!("2 unsafe fn: {second}");
    // SAFETY: this program has one thread, so no other access to `HITS` happens
    let hits = unsafe {
        HITS += 1;
        HITS // a copy, not a reference
    };
    println!("3 static mut: {hits}");
    println!("4 unsafe trait: {}", zeroed::<u32>());
    let bits = Bits { float: 1.0 };
    // SAFETY: every 4-byte pattern is a valid u32
    let int = unsafe { bits.int };
    println!("5 union field: {int}");
}
