fn main() {
    let a = -7 / 2;
    let b = -7 % 3;
    let c = (-7_i32).rem_euclid(3);
    let d = 7 / 2 * 2;
    let e = 0.1 + 0.2 == 0.3;
    println!("{a} {b} {c} {d} {e}");
}
