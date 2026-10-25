fn main() {
    let mut list = Vec::new();
    list.push(1);
    list.push(2);
    let list = list; // the same list, now frozen
    list.push(3);
}
