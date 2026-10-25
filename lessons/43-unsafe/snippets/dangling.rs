fn main() {
    let p: *const i32;
    {
        let x = 5;
        p = &raw const x;
    }
    // `x` is gone. Making the pointer and checking it for null are allowed; reading it is not.
    println!("null: {}", p.is_null());
}
