mod shapes {
    fn area(w: i32, h: i32) -> i32 {
        w * h
    }
}

fn main() {
    println!("{}", shapes::area(3, 4));
}
