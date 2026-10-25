fn largest<T: PartialOrd>(list: &[T]) -> T {
    let mut best = list[0];
    for item in list {
        if *item > best {
            best = *item;
        }
    }
    best
}

fn main() {
    println!("largest: {}", largest(&[3, 9, 4]));
}
