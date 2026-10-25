fn main() {
    let mut items = vec![1, 2, 3];
    let mut extra = Vec::new();
    for x in &items {
        if *x == 2 {
            extra.push(20);
        }
    }
    items.extend(extra);
    println!("{items:?}");
}
