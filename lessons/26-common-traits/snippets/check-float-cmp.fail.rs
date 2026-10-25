// f64 has no cmp: Go's cmp.Compare on floats has no a.cmp(&b) twin. Not shown on the page.
fn main() {
    let a = 1.0_f64;
    let b = 2.0_f64;
    println!("{:?}", a.cmp(&b));
}
