fn add_item(items: &mut Vec<i32>) {
    items.push(0);
}

fn main() {
    let mut items = vec![1, 2];
    add_item(&mut items);
    println!("{items:?}");
}
