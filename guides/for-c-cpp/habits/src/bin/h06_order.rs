fn main() {
    println!("{}", square(7));
    println!("{}", shapes::area(3, 4));
}

fn square(x: i32) -> i32 {
    x * x
}

mod shapes {
    pub fn area(w: i32, h: i32) -> i32 {
        w * h
    }
}
