// T can be any type that can be compared with < and >
fn largest<T: PartialOrd>(items: &[T]) -> &T {
    let mut best = &items[0];
    for item in items {
        if item > best {
            best = item;
        }
    }
    best
}

fn main() {
    println!("{}", largest(&[3, 9, 2]));
    println!("{}", largest(&["kiwi", "apple", "pear"]));
}
