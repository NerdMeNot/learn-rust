fn main() {
    // what `for i in 0..3 { ... }` does: ask for the next item until there is none
    let mut it = 0..3;
    loop {
        match it.next() {
            Some(i) => println!("manual: {i}"),
            None => break,
        }
    }

    // a range is an ordinary value with fields and methods
    let r = 2..5;
    println!("start: {}, end: {}", r.start, r.end);
    println!("len: {}", r.len());
    println!("contains 4: {}", r.contains(&4));
    println!("contains 5: {}", r.contains(&5));
}
