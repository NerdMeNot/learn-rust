fn elif(x: i32) {
    if x > 1 {
        println!("big");
    } elif x > 0 {
        println!("small");
    }
}

fn both(a: bool, b: bool) -> bool {
    a and b
}

fn invert(a: u8) -> u8 {
    ~a
}

fn main() {}
