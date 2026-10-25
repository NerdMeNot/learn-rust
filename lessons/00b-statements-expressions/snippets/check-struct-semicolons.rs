// Items: a braced struct ends at `}`; a tuple struct and a unit struct end with `;`.
struct Point {
    x: i32,
    y: i32,
}
struct Meters(f64);
struct Marker;

fn main() {
    let p = Point { x: 1, y: 2 };
    let m = Meters(2.5);
    let _k = Marker;
    // block-like statements need no `;`
    if p.x > 0 {
        println!("positive");
    }
    println!("{} {}", p.x + p.y, m.0);
}
