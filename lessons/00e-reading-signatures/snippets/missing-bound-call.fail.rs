fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut best = &list[0];
    for item in list {
        if item > best {
            best = item;
        }
    }
    best
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let points = [Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
    let best = largest(&points);
    println!("x: {}, y: {}", best.x, best.y);
}
