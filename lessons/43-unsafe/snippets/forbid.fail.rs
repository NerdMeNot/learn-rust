#![forbid(unsafe_code)]

fn main() {
    let x = 7;
    let p = &raw const x;
    // SAFETY: `p` points at `x`, which is alive
    println!("{}", unsafe { *p });
}
