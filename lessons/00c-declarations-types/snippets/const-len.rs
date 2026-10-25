const WIDTH: usize = 3;
const CELLS: usize = WIDTH * 2; // a constant can use other constants

static GREETING: &str = "hello";

fn main() {
    let grid = [0u8; CELLS];
    println!("cells: {}", grid.len());

    let a = &GREETING;
    let b = &GREETING;
    println!("same address: {}", std::ptr::eq(a, b));
}
