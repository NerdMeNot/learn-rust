// Possible: write the results through `&mut` parameters, as C does.
fn divmod_into(a: i32, b: i32, q: &mut i32, r: &mut i32) {
    *q = a / b;
    *r = a % b;
}

// Usual: return a tuple.
fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    // Rust makes you give q and r a value before the call.
    let (mut q, mut r) = (0, 0);
    divmod_into(17, 5, &mut q, &mut r);
    println!("out parameters: q: {q}, r: {r}");

    let (q, r) = divmod(17, 5);
    println!("returned tuple: q: {q}, r: {r}");
}
