struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let (a, b) = (1, "two");
    let Point { x, y } = Point { x: 3, y: 4 };
    let [first, _, third] = [10, 20, 30];
    let (lo, _) = (5, 6); // `_` ignores a part

    println!("tuple: {a} {b}");
    println!("struct: {x} {y}");
    println!("array: {first} {third}");
    println!("lo: {lo}");
}
