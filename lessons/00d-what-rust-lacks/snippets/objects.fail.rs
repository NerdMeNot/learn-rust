struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn bump(&mut self) {
        this.x += 1;
    }

    fn x() -> i32 {
        self.x
    }
}

fn main() {
    let p = Point(1, 2);
    println!("{}", p.x);
}
