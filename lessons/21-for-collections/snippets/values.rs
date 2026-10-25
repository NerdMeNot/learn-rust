fn main() {
    let nums = vec![10, 20, 30];

    let mut total = 0;
    for x in &nums {
        total += *x; // `*x` follows the reference to the i32
    }
    println!("with *x: {total}");

    let mut total = 0;
    for &x in &nums {
        total += x; // the pattern `&x` does the same, once per item
    }
    println!("with &x: {total}");

    for x in &nums {
        println!("x + 1: {}", x + 1); // arithmetic works on a &i32 directly
    }
}
