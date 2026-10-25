fn main() {
    let mut items = vec![1, 2, 3];
    for i in 0..items.len() {
        if items[i] == 2 {
            items.push(20);
        }
    }
    println!("{items:?}");
}
