// Backs the claim: `Vec::iter(&v)` fails because `iter` is a slice method, but the slice path works.
fn main() {
    let v = vec![1, 2, 3];
    let total: i32 = <[i32]>::iter(&v).sum();
    assert_eq!(total, 6);
    println!("checks passed");
}
