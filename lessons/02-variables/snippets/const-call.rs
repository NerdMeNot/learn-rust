const fn square(n: u32) -> u32 {
    n * n
}

const AREA: u32 = square(4);

fn main() {
    println!("area: {AREA}");
}
