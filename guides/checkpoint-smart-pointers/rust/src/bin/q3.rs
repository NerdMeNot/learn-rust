use std::rc::Rc;

fn main() {
    let mut mine = Rc::new(vec![1, 2]);
    let theirs = Rc::clone(&mine);
    Rc::make_mut(&mut mine).push(3);
    println!("{:?} {:?}", mine, theirs);
    println!("{} {}", Rc::strong_count(&mine), Rc::strong_count(&theirs));
    Rc::make_mut(&mut mine).push(4);
    println!("{:?}", mine);
}
