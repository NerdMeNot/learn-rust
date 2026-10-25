use std::sync::Arc;

fn main() {
    let list = Arc::new(vec![1, 2, 3]);
    list.push(4);
    println!("{list:?}");
}
