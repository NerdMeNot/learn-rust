fn pair(early: bool) -> (i32, i32) {
    if early {
        return 1, 2;
    }
    (3, 4)
}

fn main() {
    println!("{:?}", pair(true));
}
