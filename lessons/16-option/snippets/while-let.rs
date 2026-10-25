fn main() {
    let mut stack = vec![1, 2, 3];

    // pop() returns Some(last item), or None once the list is empty.
    while let Some(top) = stack.pop() {
        println!("popped {top}");
    }
    println!("left over: {stack:?}");
    println!("pop on empty: {:?}", stack.pop());
}
