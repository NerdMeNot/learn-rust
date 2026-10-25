/// Generic over the element type "in case" a second caller shows up.
fn first_or_zero<T: Copy + Into<u32>>(items: &[T]) -> u32 {
    items.first().map_or(0, |&x| x.into())
}

fn main() {
    println!("{}", first_or_zero(&[7u32, 8]));
    println!("{}", first_or_zero(&[]));
}
