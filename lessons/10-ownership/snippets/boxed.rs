// A list where each node holds the next one. Without Box, Node would have to
// contain itself, and its size would be infinite. A Box is one pointer.
enum List {
    Node(i32, Box<List>),
    End,
}

fn sum(list: &List) -> i32 {
    match list {
        List::Node(value, next) => value + sum(next),
        List::End => 0,
    }
}

fn main() {
    let b = Box::new(41); // one i32, on the heap
    let n = *b + 1; // * reads the value inside
    println!("b {b}, n {n}");

    let list = List::Node(1, Box::new(List::Node(2, Box::new(List::End))));
    println!("sum of list: {}", sum(&list));

    let moved = list; // moves the outer node; the heap nodes stay where they are
    println!("size_of List: {} bytes", size_of_val(&moved));
} // moved is dropped, and with it every Box down the chain
