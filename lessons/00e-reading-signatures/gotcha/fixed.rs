fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut best = &list[0];
    for item in list {
        if item > best {
            best = item;
        }
    }
    best
}

fn main() {
    let numbers = [3, 9, 4];
    let words = ["pear", "fig", "apple"];
    println!("largest number: {}", largest(&numbers));
    println!("largest word: {}", largest(&words));
}
