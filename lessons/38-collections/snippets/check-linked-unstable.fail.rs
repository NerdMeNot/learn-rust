use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::from([1, 2, 3, 4]);
    let third = list.remove(2);
    let mut cursor = list.cursor_front_mut();
    cursor.insert_after(9);
    println!("{third:?}");
}
