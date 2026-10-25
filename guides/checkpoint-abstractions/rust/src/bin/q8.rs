fn biggest<T: PartialOrd>(items: &[T]) -> T {
    let mut best = items[0];
    for item in items {
        if *item > best {
            best = *item;
        }
    }
    best
}

fn main() {
    let words = [String::from("pear"), String::from("zebra")];
    println!("{}", biggest(&words));
}
