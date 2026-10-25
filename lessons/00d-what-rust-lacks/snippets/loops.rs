fn main() {
    // count 0, 1, 2
    for i in 0..3 {
        println!("i: {i}");
    }

    // count down from 10 in steps of 3
    for i in (1..=10).rev().step_by(3) {
        println!("down: {i}");
    }

    // the position and the item
    let items = ["a", "b"];
    for (i, item) in items.iter().enumerate() {
        println!("item {i}: {item}");
    }

    // repeat while there is a value
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped: {top}");
    }

    // run the body first, then test
    let mut n = 0;
    loop {
        n += 1;
        if n >= 3 {
            break;
        }
    }
    println!("n: {n}");
}
