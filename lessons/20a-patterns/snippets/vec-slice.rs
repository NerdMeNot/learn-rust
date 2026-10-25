fn main() {
    // a slice pattern needs an array or a slice, so slice the Vec first
    let words = vec!["a", "b", "c"];
    if let [first, rest @ ..] = words.as_slice() {
        println!("first = {first}, rest = {rest:?}");
    }

    let mut stack = vec![7];
    for _ in 0..2 {
        match stack[..] {
            [] => println!("empty"),
            [only] => println!("one value: {only}"),
            [.., top] => println!("top of {}: {top}", stack.len()),
        }
        stack.push(8);
    }
}
