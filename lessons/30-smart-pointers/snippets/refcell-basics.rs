use std::cell::RefCell;

fn main() {
    let list = RefCell::new(vec![1, 2]); // not `mut`: it is changed through &
    list.borrow_mut().push(3);
    println!("list: {:?}", list.borrow());

    let reader = list.borrow(); // a shared borrow, alive until `reader` goes
    println!("second reader ok: {}", list.try_borrow().is_ok());
    println!("writer ok: {}", list.try_borrow_mut().is_ok());
    println!("the refusal: {:?}", list.try_borrow_mut().err());
    drop(reader);
    println!("writer ok now: {}", list.try_borrow_mut().is_ok());

    println!("size_of RefCell<()>: {}", size_of::<RefCell<()>>());
    println!("size_of Vec<i32>: {}", size_of::<Vec<i32>>());
    println!(
        "size_of RefCell<Vec<i32>>: {}",
        size_of::<RefCell<Vec<i32>>>()
    );
}
