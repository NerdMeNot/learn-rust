/// One caller today, so one concrete signature. `&[u32]` accepts a Vec, an array and `&[]`.
fn first_or_zero(items: &[u32]) -> u32 {
    items.first().copied().unwrap_or(0)
}

fn main() {
    println!("{}", first_or_zero(&[7, 8]));
    println!("{}", first_or_zero(&vec![9]));
    println!("{}", first_or_zero(&[]));
}
