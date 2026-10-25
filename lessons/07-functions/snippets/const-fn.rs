const fn square(n: u32) -> u32 {
    n * n
}

const AREA: u32 = square(12);
static GRID: [u8; square(3) as usize] = [0; square(3) as usize];

fn main() {
    println!("AREA: {AREA}");
    println!("GRID.len(): {}", GRID.len());
    println!("square(7) at run time: {}", square(7));
}
