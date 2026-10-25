fn add_item(items: &Vec<i32>) {
    items.push(0);
}

fn main() {
    let mut items = vec![1, 2, 3];
    add_item(&items);
    println!("items: {items:?}");
}
