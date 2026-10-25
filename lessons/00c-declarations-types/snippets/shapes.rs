struct Point {
    x: i32,
    y: i32,
}

struct Meters(f64);

fn main() {
    let p = Point { x: 3, y: 4 };
    let m = Meters(2.5);
    let pair: (i32, &str) = (7, "seven");
    let grid: [i32; 3] = [10, 20, 30];
    let zeros = [0u8; 4];
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let middle: &[i32] = &v[1..4];

    println!("point: {} {}", p.x, p.y);
    println!("meters: {}", m.0);
    println!("tuple: {} {}", pair.0, pair.1);
    println!("array: {grid:?} {zeros:?}");
    println!("slice: {middle:?}");
    println!("ranges: {:?} {:?} {:?} {:?}", 0..5, 0..=5, 2.., ..3);
}
