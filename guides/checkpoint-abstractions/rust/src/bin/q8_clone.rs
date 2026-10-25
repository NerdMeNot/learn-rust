fn biggest<T: PartialOrd + Clone>(items: &[T]) -> T {
    let mut best = items[0].clone();
    for item in items {
        if *item > best {
            best = item.clone();
        }
    }
    best
}

fn main() {
    let words = [String::from("pear"), String::from("zebra")];
    println!("{}", biggest(&words));
}
