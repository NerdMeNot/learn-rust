fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!("{}", area(width = 3, height = 4));
    println!("{}", area(3));
}
