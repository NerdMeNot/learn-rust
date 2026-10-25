fn size(n: Option<u32>) -> String {
    match n {
        // an or-pattern can sit inside another pattern
        Some(1 | 2) => String::from("one or two"),
        // `@` names the whole value and still tests the shape
        Some(small @ 3..=9) => format!("small: {small}"),
        Some(other) => format!("big: {other}"),
        None => String::from("missing"),
    }
}

fn main() {
    for n in [Some(2), Some(5), Some(50), None] {
        println!("{n:?} -> {}", size(n));
    }

    // both sides of | must bind the same names, with the same types
    let results = [Ok(3), Err(4)];
    for r in results {
        let (Ok(n) | Err(n)) = r;
        println!("{r:?} -> n = {n}");
    }
}
