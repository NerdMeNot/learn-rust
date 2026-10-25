fn main() {
    print!("print! has no newline; ");
    println!("println! adds one");
    let s = format!("{}-{}", 1, 2);
    println!("format! returns a String: {s}");
    let v = vec![10, 20, 30];
    println!("vec! builds a Vec: {v:?}");
    assert!(v.len() == 3);
    assert_eq!(v[0], 10);
    let c = 'k';
    println!(
        "matches! is a yes/no pattern test: {}",
        matches!(c, 'a'..='z' | 'A'..='Z')
    );
    let doubled = dbg!(v[1] * 2);
    println!("dbg! returned the value: {doubled}");
}
