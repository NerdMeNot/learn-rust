fn main() {
    let s = String::from("hello");
    let pair = (String::from("left"), String::from("right"));

    let _ = s; // `_` does not bind, so nothing moves
    println!("after `let _ = s`, s is still {s}");

    let (_, right) = pair; // only the right half moves
    println!("after `let (_, right) = pair`, pair.0 is still {}", pair.0);
    println!("right is {right}");
}
