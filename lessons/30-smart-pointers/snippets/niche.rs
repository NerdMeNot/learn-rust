use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    println!("Box<i32>: {}", size_of::<Box<i32>>());
    println!("Option<Box<i32>>: {}", size_of::<Option<Box<i32>>>());
    println!("Option<&i32>: {}", size_of::<Option<&i32>>());
    println!("Option<Rc<i32>>: {}", size_of::<Option<Rc<i32>>>());
    println!("i32: {}", size_of::<i32>());
    println!("Option<i32>: {}", size_of::<Option<i32>>());
    println!("Option<u64>: {}", size_of::<Option<u64>>());
    println!("Node: {}", size_of::<Node>());

    let list = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: None,
        })),
    };
    println!("second: {:?}", list.next.as_ref().map(|n| n.value));
}
