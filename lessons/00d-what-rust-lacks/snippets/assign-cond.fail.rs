fn main() {
    let mut items = vec![1, 2, 3].into_iter();
    let mut item;
    while item = items.next() {
        println!("{item:?}");
    }
}
