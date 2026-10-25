use std::sync::Mutex;

fn main() {
    let list = Mutex::new(vec![1, 2, 3]);
    list.push(4);
}
