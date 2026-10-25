fn main() {
    let mut a = 1;
    let mut b = 2;
    (a, b) = (b, a);
    println!("after swap: a: {a}, b: {b}");

    // Every new value is computed from the old ones, so rotating three works the same way.
    let (mut x, mut y, mut z) = (1, 2, 3);
    (x, y, z) = (y, z, x);
    println!("rotated: x: {x}, y: {y}, z: {z}");
}
