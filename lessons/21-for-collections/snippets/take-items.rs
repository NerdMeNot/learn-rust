// `for msg in outbox` would give `&mut String`, not owned strings.
// take moves the whole list out and leaves an empty one in its place.
fn send_all(outbox: &mut Vec<String>) {
    for msg in std::mem::take(outbox) {
        println!("sent {msg}");
    }
}

fn main() {
    let mut names = vec![String::from("ana"), String::from("bo"), String::from("cy")];

    // drain(..2) moves the first two items out and leaves the rest in the list.
    for n in names.drain(..2) {
        println!("drained {n}");
    }
    println!("left in the list: {names:?}");

    send_all(&mut names);
    println!("names after send_all: {names:?}");
}
