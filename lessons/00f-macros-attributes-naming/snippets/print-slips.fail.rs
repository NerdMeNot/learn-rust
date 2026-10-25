struct Point {
    x: i32,
}

fn main() {
    let count = 5;
    let p = Point { x: 1 };
    println!("count:", count);
    println!(count);
    println!("x is {p.x}");
    println!("count: " + count);
}
