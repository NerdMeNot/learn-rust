use std::rc::Rc;

fn main() {
    let mut a = Rc::new(vec![1, 2, 3]);
    println!("owners: {}", Rc::strong_count(&a));
    println!("size_of Rc<Vec<i32>>: {}", size_of::<Rc<Vec<i32>>>());

    {
        let b = Rc::clone(&a); // a second owner of the same Vec
        println!("owners: {}", Rc::strong_count(&a));
        println!("same Vec: {}", Rc::ptr_eq(&a, &b));
        println!("same buffer: {}", a.as_ptr() == b.as_ptr());
        println!("get_mut while shared: {:?}", Rc::get_mut(&mut a));
    } // b goes: the count drops, the Vec stays

    println!("owners: {}", Rc::strong_count(&a));
    println!("get_mut when alone: {:?}", Rc::get_mut(&mut a));

    let copy: Vec<i32> = (*a).clone(); // this one really copies the numbers
    println!("copy has its own buffer: {}", copy.as_ptr() != a.as_ptr());
}
