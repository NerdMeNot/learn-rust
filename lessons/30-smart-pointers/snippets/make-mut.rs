use std::rc::Rc;

fn main() {
    let mut mine = Rc::new(vec![1, 2, 3]);
    let theirs = Rc::clone(&mine);
    println!("owners: {}", Rc::strong_count(&mine));

    // Rc::make_mut: change the value; if others share it, copy it first
    Rc::make_mut(&mut mine).push(4);
    println!("mine: {:?}", mine);
    println!("theirs: {:?}", theirs);
    println!(
        "owners now: {} and {}",
        Rc::strong_count(&mine),
        Rc::strong_count(&theirs)
    );
    println!("same Vec: {}", Rc::ptr_eq(&mine, &theirs));

    // alone, it changes the value in place
    drop(theirs);
    let before = Rc::as_ptr(&mine);
    Rc::make_mut(&mut mine).push(5);
    println!("mine: {:?}", mine);
    println!("same allocation: {}", Rc::as_ptr(&mine) == before);
}
