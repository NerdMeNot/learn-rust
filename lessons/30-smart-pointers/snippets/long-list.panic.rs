struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

fn main() {
    let mut head: Option<Box<Node>> = None;
    for value in 0..1_000_000 {
        head = Some(Box::new(Node { value, next: head }));
    }

    let mut len = 0;
    let mut at = &head;
    while let Some(node) = at {
        len += 1;
        at = &node.next;
    }
    println!(
        "built a list of {len}, top value {}",
        head.as_ref().map_or(0, |n| n.value)
    );
} // head is dropped here: one Box frees the next, which frees the next, ...
